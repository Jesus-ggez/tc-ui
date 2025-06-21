//<·
pub fn repr(val: &str) -> String {
    let needs_quotes = val.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_' && c != '#');

    if needs_quotes {
        return format!("\"{}\"", val);
    }

    val.to_string()
}
