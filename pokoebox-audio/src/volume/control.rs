use alsa::mixer::{Mixer, Selem, SelemChannelId, SelemId};

const DEFAULT_CHANNEL: SelemChannelId = SelemChannelId::FrontLeft;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ControlHandle(u32, Option<String>);

#[derive(Clone, Debug)]
pub struct ControlProps {
    /// Control name.
    pub name: Option<String>,

    /// The value at initialization.
    pub init_value: i64,

    /// Control range.
    pub range: (i64, i64),
}

pub(crate) struct Control {
    /// Handle.
    handle: ControlHandle,

    /// Alsa element ID.
    selem: SelemId,

    /// Control properties.
    props: ControlProps,
}

impl Control {
    /// Create control from Alsa Selem.
    pub fn from_selem<'a>(selem: Selem<'a>) -> Self {
        let id = selem.get_id();
        let props = ControlProps {
            name: id.get_name().ok().map(|n| n.into()),
            init_value: selem
                .get_playback_volume(DEFAULT_CHANNEL)
                .expect("failed to query Alsa control volume"),
            range: selem.get_playback_volume_range(),
        };

        let handle = ControlHandle(id.get_index(), id.get_name().map(|n| n.into()).ok());

        Self {
            handle,
            selem: id,
            props,
        }
    }

    /// Get handle to this control.
    pub fn handle(&self) -> &ControlHandle {
        &self.handle
    }

    /// Get control properties.
    pub fn props(&self) -> &ControlProps {
        &self.props
    }

    /// Get reference to Alsa Selem.
    pub fn selem<'a>(&self, mixer: &'a Mixer) -> Selem<'a> {
        // TODO: do not unwrap here
        mixer
            .find_selem(&self.selem)
            .expect("failed to get selem from Alsa mixer")
    }

    /// Get current volume.
    pub fn get_volume(&self, mixer: &Mixer) -> i64 {
        self.selem(mixer)
            .get_playback_volume(DEFAULT_CHANNEL)
            .expect("failed to set Alsa control volume")
    }

    /// Set current volume.
    pub fn set_volume(&self, mixer: &Mixer, volume: i64) -> alsa::Result<()> {
        self.selem(mixer).set_playback_volume_all(volume)
    }
}
