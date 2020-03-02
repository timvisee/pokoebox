use alsa::card::Card;
use alsa::mixer::{Mixer, Selem, SelemChannelId};

fn main() {
    let card: Card = alsa::card::Iter::new().nth(1).unwrap().unwrap();
    eprintln!("Card: longname: {:?}", card.get_longname());
    eprintln!("Card: name: {:?}", card.get_name());
    eprintln!("Card: index: {:?}", card.get_index());
    let id = format!("hw:{}", card.get_index());

    let mixer = Mixer::new(&id, true).expect("failed to open mixer");
    eprintln!("Mixer: {:?}", mixer);

    for elem in mixer.iter() {
        let selem = Selem::new(elem).expect("failed to wrap elem as selem");

        if selem.has_playback_volume() {
            eprintln!("Selem: {:?}", elem);
            let volume = selem
                .get_playback_volume(SelemChannelId::FrontLeft)
                .expect("failed to get");
            let (low, high) = selem.get_playback_volume_range();
            let id = selem.get_id();
            eprintln!("- ID: {}", id.get_name().expect("failed to get name"),);
            eprintln!("- Volume: {} ({} - {})", volume, low, high);
            // eprintln!("- Has volume: {:?}", selem.has_volume());
        }
    }
}
