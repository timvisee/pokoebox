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

    use bluez::client::{DiscoverableMode, IoCapability};

    let result = client
        .set_local_name(controller, "PokoeBox", Some("PokoeBox"))
        .await;
    eprintln!("Local name result: {:?}", result,);

    // eprintln!(
    //     "Features: {:?}",
    //     client.get_advertising_features(controller))
    // );

    let result = client.set_fast_connectable(controller, true).await;
    eprintln!("Fast connectable result: {:?}", result);

    let result = client.set_ssp(controller, true).await;
    eprintln!("SSP result: {:?}", result);

    let result = client.set_high_speed(controller, true).await;
    eprintln!("High speed result: {:?}", result);

    // TODO: set_le ?

    let result = client
        .set_io_capability(controller, IoCapability::DisplayOnly)
        .await;
    eprintln!("IO result: {:?}", result);

    use bluez::interface::class::{AudioVideoDeviceClass, DeviceClass};
    let result = client
        .set_device_class(
            controller,
            DeviceClass::AudioVideo(AudioVideoDeviceClass::Loudspeaker),
        )
        .await;
    eprintln!("Class result: {:?}", result);

    let result = client.set_bondable(controller, true).await;
    eprintln!("Bondable result: {:?}", result);

    let result = client.set_connectable(controller, true).await;
    eprintln!("Connectable result: {:?}", result);

    let result = client
        .set_discoverable(controller, DiscoverableMode::General, Some(60))
        .await;
    eprintln!("Discoverable result: {:?}", result);

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
                eprintln!("Confirming (hint: {}, value: {})...", confirm_hint, value);
                let result = client
                    .user_confirmation_reply(controller, address, address_type, true)
                    .await;
                eprintln!("Confirm reply result: {:?}", result);
            }
            Event::NewLinkKey {
                store_hint: bool,
                address,
                address_type,
                key_type,
                value,
                pin_length,
            } => {}
            e @ Event::AuthenticationFailed { .. } => {
                eprintln!(">>> AUTH FAILED: {:?}", e);
            }
            e => {
                eprintln!(">>> UNKNWN EVENT: {:?}", e);
            } // Event::DeviceFound {
              //     address,
              //     address_type,
              //     flags,
              //     rssi,
              //     ..
              // } => {
              //     println!(
              //         "[{:?}] found device {} ({:?})",
              //         controller, address, address_type
              //     );
              //     println!("\tflags: {:?}", flags);
              //     println!("\trssi: {:?}", rssi);
              // }
              // Event::Discovering {
              //     discovering,
              //     address_type,
              // } => {
              //     println!("discovering: {} {:?}", discovering, address_type);

              //     // if discovery ended, turn it back on
              //     if !discovering {
              //         client
              //             .start_discovery(
              //                 controller,
              //                 AddressTypeFlag::BREDR
              //                     | AddressTypeFlag::LEPublic
              //                     | AddressTypeFlag::LERandom,
              //             )
              //             .await?;
              //     }
              // }
        }

        match client.get_connections(controller).await {
            Ok(cons) => {
                eprintln!("# CONS ({}): {:?}", cons.len(), cons);
            }
            Err(err) => {
                eprintln!("# CON INDEX ERR: {:?}", err);
            }
        }

        std::thread::sleep(Duration::from_millis(50));
    }

    // scan for some devices
    // to do this we'll need to listen for the Device Found event

    // client
    //     .start_discovery(
    //         controller,
    //         AddressTypeFlag::BREDR | AddressTypeFlag::LEPublic | AddressTypeFlag::LERandom,
    //     )
    //     .await?;

    // // just wait for discovery forever
    // loop {
    //     // process() blocks until there is a response to be had
    //     let response = client.process().await?;

    //     match response.event {
    //         Event::DeviceFound {
    //             address,
    //             address_type,
    //             flags,
    //             rssi,
    //             ..
    //         } => {
    //             println!(
    //                 "[{:?}] found device {} ({:?})",
    //                 controller, address, address_type
    //             );
    //             println!("\tflags: {:?}", flags);
    //             println!("\trssi: {:?}", rssi);
    //         }
    //         Event::Discovering {
    //             discovering,
    //             address_type,
    //         } => {
    //             println!("discovering: {} {:?}", discovering, address_type);

    //             // if discovery ended, turn it back on
    //             if !discovering {
    //                 client
    //                     .start_discovery(
    //                         controller,
    //                         AddressTypeFlag::BREDR
    //                             | AddressTypeFlag::LEPublic
    //                             | AddressTypeFlag::LERandom,
    //                     )
    //                     .await?;
    //             }
    //         }
    //         _ => (),
    //     }

    //     std::thread::sleep(Duration::from_millis(50));
    // }
}
