use std::io::Cursor;

/// Sound to play at startup.
const STARTUP_SOUND: Option<Sound> = Some(Sound::MustangStart);

use rodio::{
    decoder::{Decoder, DecoderError},
    Device, Source,
};

/// System to play sound effects.
pub struct SoundEffecter {
    /// Output device.
    device: Device,
}

impl SoundEffecter {
    /// Construct new sound effecter.
    pub fn new() -> Result<Self, Error> {
        // Select output device, build effecter
        let effecter = Self {
            device: rodio::default_output_device().ok_or(Error::NoOutput)?,
        };

        // Play startup sound
        if let Some(sound) = STARTUP_SOUND {
            if let Err(err) = effecter.play(sound) {
                error!("Failed to play startup sound: {:?}", err);
            }
        }

        Ok(effecter)
    }

    /// Play given sound effect.
    pub fn play(&self, sound: Sound) -> Result<(), Error> {
        Ok(rodio::play_raw(
            &self.device,
            sound_source(sound)?.convert_samples(),
        ))
    }
}

/// Available sound effect types.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sound {
    Kick,
    Guitar,
    Xp,
    Jbl,
    MustangStart,
}

/// Get a decoder for the given sound effect type.
fn sound_source(sound: Sound) -> Result<Decoder<Cursor<&'static [u8]>>, Error> {
    // Select sound
    let sound: &[u8] = match sound {
        Sound::Kick => include_bytes!("../../res/sounds/kick_30hz.ogg"),
        Sound::Guitar => include_bytes!("../../res/sounds/guitar.ogg"),
        Sound::Xp => include_bytes!("../../res/sounds/xp.ogg"),
        Sound::Jbl => include_bytes!("../../res/sounds/jbl.ogg"),
        Sound::MustangStart => include_bytes!("../../res/sounds/mustang_start_long.ogg"),
    };

    // Build source
    rodio::Decoder::new(Cursor::new(sound)).map_err(Error::Decode)
}

/// An sound effecter error.
#[derive(Debug, Clone)]
pub enum Error {
    /// Could not select audio output device.
    NoOutput,

    /// An error occurred while decoding audio.
    Decode(DecoderError),
}
