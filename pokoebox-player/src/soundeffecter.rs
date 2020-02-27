use std::io::Cursor;

use rodio::{Decoder, Device, Source};

/// System to play sound effects.
pub struct SoundEffecter {
    /// Output device.
    device: Device,
}

impl SoundEffecter {
    pub fn new() -> Self {
        // Select output device
        // TODO: do not unwrap
        let device = rodio::default_output_device().expect("failed to select output device");

        // Build effecter
        let effecter = Self { device };

        // Play startup sound
        effecter.play(Sound::MustangStart);

        effecter
    }

    /// Play given sound effect.
    pub fn play(&self, sound: Sound) {
        rodio::play_raw(&self.device, sound_source(sound).convert_samples());
    }
}

/// Available sound types.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sound {
    Kick,
    Guitar,
    Xp,
    Jbl,
    MustangStart,
}

fn sound_source(sound: Sound) -> Decoder<Cursor<&'static [u8]>> {
    // Select sound
    let sound: &[u8] = match sound {
        Sound::Kick => include_bytes!("../../res/sounds/kick_30hz.ogg"),
        Sound::Guitar => include_bytes!("../../res/sounds/guitar.ogg"),
        Sound::Xp => include_bytes!("../../res/sounds/xp.ogg"),
        Sound::Jbl => include_bytes!("../../res/sounds/jbl.ogg"),
        Sound::MustangStart => include_bytes!("../../res/sounds/mustang_start_long.ogg"),
    };

    // Build source
    // TODO: do not unwrap
    rodio::Decoder::new(Cursor::new(sound)).expect("failed to create sound effect decoder")
}
