// TODO: do not hardcode this here
const ALSA_CARD_PREFER: &str = "Intel";

/// Select sound card to use.
pub(crate) fn select_card() -> String {
    format!(
        "hw:{}",
        alsa::card::Iter::new()
            .filter_map(|c| c.ok())
            .filter_map(|c| {
                c.get_name()
                    .ok()
                    .map(|n| Some((c.get_index(), n)))
                    .unwrap_or(None)
            })
            .filter(|(_, n)| n.contains(ALSA_CARD_PREFER))
            .inspect(|(i, n)| info!("Selected sound card: {} (hw:{})", n, i))
            .map(|(i, _)| i)
            .next()
            .unwrap_or(0)
    )
}
