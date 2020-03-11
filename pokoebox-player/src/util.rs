/// Format number to show `sig` number of significant numbers.
pub fn format_num_sig(n: f32, sig: usize) -> String {
    format!(
        "{:.*}",
        (sig as f32 - n.log10() - 0.0000001).max(0.0) as usize,
        n
    )
}
