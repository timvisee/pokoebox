#[cfg(feature = "rpi")]
extern crate cupi;
#[macro_use]
extern crate log;
extern crate pokoebox_player;

#[cfg(feature = "rpi")]
use cupi::{CuPi, delay_ms};
#[cfg(feature = "rpi")]
use cupi::board;

use pokoebox_player::app::App;
use pokoebox_player::logger::Logger;
use pokoebox_player::manifest;

fn main() {
    // Initialize the application logger
    Logger::init().expect("Failed to initialize logger.");

    // Show an initial message
    info!("Starting {} v{}...", manifest::APP_NAME, manifest::APP_VERSION_NAME);
    info!("Developed by {}.", manifest::APP_ABOUT);

    // Create a new app instance
    let mut app = App::new().expect("Failed to initialize application.");

    // Start the application
    app.start().expect("Failed to start application.");

    // Start the main loop of the application
    app.main_loop();

    #[cfg(feature = "rpi")]
    {
        use pokoebox_player::perif::perif_manager::PerifManager;
        use pokoebox_player::perif::perif_gpio_button::PerifGpioButton;
        use pokoebox_player::perif::perif_gpio_light::PerifGpioLight;
        use pokoebox_player::perif::perif_type::PerifType;
        use pokoebox_player::perif::traits::button::Button;

        // Set up CuPi
        let cupi = CuPi::new().unwrap();

        // Create a new peripheral manager
        let mut perifs = PerifManager::new();

        // Create a new button and light peripheral
        let button = PerifGpioButton::new_wrapped("My button!", 0, &cupi).unwrap();
        let light = PerifGpioLight::new_wrapped("My light!", 3, &cupi).unwrap();

        // Add the peripherals to the manager
        perifs.add_perif(button).unwrap();
        perifs.add_perif(light).unwrap();

        // Loop to invoke some actions as a test
        loop {
            // Loop through all peripherals
            for perif in perifs.perifs_mut() {
                match *perif {
                    PerifType::GpioLight(ref mut perif) => {
                        perif.toggle();
                    },
                    PerifType::GpioButton(ref perif) => {
                        if perif.is_pressed().unwrap() {
                            println!("Button pressed!");
                        } else {
                            println!("Button not pressed!");
                        }
                    }
                }
            }

            println!("Waiting 500ms...");
            delay_ms(500);
        }

        use self::pokoebox_player::gpio::pin_config::{PinConfig, IoMode};

        // Print the board we're using
        println!("Board: {:?}", board());

        // Set up CuPi
        let cupi = CuPi::new().unwrap();

        // Create a pin configuration
        let pin_config = PinConfig::new_with_pin_and_io(0, IoMode::Output);
//        let mut pin_config = PinConfig::new_with_pin_and_io(0, IoMode::Input);
//        pin_config.set_pull_mode(PullMode::PullUp);

        // Create the pint
        let mut pin = pin_config.into_pin(&cupi).unwrap();

        loop {
            println!("Pin 0: ON");
            pin.high();
            delay_ms(200);

            println!("Pin 0: OFF");
            pin.low();
            delay_ms(200);
        }

//        loop {
//            match pin.read() {
//                Logic::High => println!("Pin 0 is HIGH"),
//                Logic::Low => println!("Pin 0 is LOW"),
//            }
//            delay_ms(200);
//        }
    }
}
