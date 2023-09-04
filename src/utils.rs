/// Truncates a string to a maximum length.
///
/// It is used to avoid printing too long strings to the console.
#[must_use]
pub fn truncate_string(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect()
}
