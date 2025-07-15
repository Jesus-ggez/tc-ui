//<·
pub fn repr(val: &str) -> String {
    let quoted = vec!['-', '_', '#', ' '];
    let needs_quotes = val
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !quoted.contains(&c));

    if needs_quotes {
        return format!("\"{}\"", val);
    }

    val.to_string()
}
