
pub fn str2path(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c == ' ' {'-'} else {c})
        .filter(|c| c == &'-' || c.is_alphanumeric())
        .collect()
}