/// Generates a bar representing how full a mountpoint is
pub(crate) fn generate(percent_full: u64, segments: u8) -> String {
    let empty = "░";
    let full = "█";
    let mut out = String::new();

    let full_segments = (segments as f64 * (percent_full as f64 / 100.0)) as usize;

    out.push_str(&full.repeat(full_segments));
    out.push_str(&empty.repeat(segments as usize - full_segments));


    out
}