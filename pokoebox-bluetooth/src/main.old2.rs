use std::error::Error;
use std::time::Duration;

use async_std::task::block_on;

use bluez::client::*;
use bluez::interface::controller::*;
use bluez::interface::event::Event;

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = BlueZClient::new().unwrap();

    let controllers = client.get_controller_list().await?;

    // find the first controller we can power on
    let (controller, info) = controllers
        .into_iter()
        .filter_map(|controller| {
            let info = block_on(client.get_controller_info(controller)).ok()?;

            if info.supported_settings.contains(ControllerSetting::Powered) {
                Some((controller, info))
            } else {
                None
            }
        })
        .nth(0)
        .expect("no usable controllers found");

    if !info.current_settings.contains(ControllerSetting::Powered) {
        println!("powering on bluetooth controller {}", controller);
        client.set_powered(controller, true).await?;
    }

    client
        .set_local_name(controller, "PokoeBox", Some("PokoeBox"))
        .await?;

    client
        .set_io_capability(controller, IoCapability::NoInputNoOutput)
        .await?;

    // Class: 0x0020041C
    // Class: 0x000c041c
    // let result = client
    //     .set_device_class(
    //         controller,
    //         DeviceClass::AudioVideo(AudioVideoDeviceClass::Portable),
    //     )
    //     .await;
    // eprintln!("Class result: {:?}", result);

    // let result = client.set_bondable(controller, true).await;
    // eprintln!("Bondable result: {:?}", result);

    client.set_connectable(controller, true).await?;

    client
        .set_discoverable(controller, DiscoverableMode::General, Some(60))
        .await?;

    // just wait for discovery forever
    loop {
        // process() blocks until there is a response to be had
        let response = client.process().await?;

        match response.event {
            Event::DeviceConnected { .. } => {}
            Event::UserConfirmationRequest {
                address,
                address_type,
                confirm_hint,
                value,
            } => {
                eprintln!("Adding device!");
                let result = client
                    .add_device(
                        controller,
                        address,
                        address_type,
                        AddDeviceAction::AllowConnect,
                    )
                    .await?;
                eprintln!("Device add result: {:?}", result);

                eprintln!("Confirming (hint: {}, value: {})...", confirm_hint, value);
                let result = client
                    .user_confirmation_reply(controller, address, address_type, true)
                    .await;
                eprintln!("Confirm reply result: {:?}", result);
            }
            e => {
                eprintln!(">>> EVENT: {:?}", e);
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
