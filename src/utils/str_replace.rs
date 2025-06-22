//<·
pub fn replace(s: &str, old: &str, new: &str, limit: Option<usize>) -> String {
    if old.is_empty() {
        return s.to_string();
    }

    let lim = limit.unwrap_or(usize::MAX);
    let mut result = "".to_string();
    let mut remain = s;
    let mut count = 0;

    while count < lim {
        if let Some(pos) = remain.find(old) {
            result.push_str(&remain[..pos]);
            result.push_str(new);

            remain = &remain[pos + old.len()..];
            count += 1;
        } else {
            break;
        }
    }
    result.push_str(remain);
    result
}
