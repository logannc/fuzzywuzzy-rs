use std::ascii::AsciiExt;

pub fn full_process(s: &str, force_ascii: bool) -> String {
    let mut result = s.to_string();
    if force_ascii {
        result = result.chars().filter(AsciiExt::is_ascii).collect();
    }
    result = result.chars().map(|c| if c.is_alphanumeric() { c } else { ' ' }).collect();
    result.make_ascii_lowercase();
    result.trim().to_string()
}