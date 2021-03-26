//! Choice ported functions from fuzzywuzzy.utils (if they make sense)
//!
//! For example, `fuzzywuzzy.utils.validate_string` doesn't make much sense because we do not need to validate the type.

use crate::fuzzywuzzy_compatible::string_processing::replace_non_letters_non_numbers_with_whitespace;

/// Returns a [String] composed of all of the ASCII pieces of the input string.
///
/// Note that this function does not include extended ASCII characters.
/// ```
/// # use fuzzywuzzy::fuzzywuzzy_compatible::utils::asciionly;
/// assert_eq!(asciionly("abc123"), "abc123");
/// assert_eq!(asciionly("abcØØØ"), "abc");
/// assert_eq!(asciionly("abcØØØकिमपि"), "abc");
/// assert_eq!(asciionly("ØØØकिमपि"), "");
/// ```
pub fn asciionly(s: &str) -> String {
    s.chars().filter(char::is_ascii).collect()
}

/// Process string by removing all but letters and numbers, force to lowercase, trim whitespace.
///
/// If `force_ascii`, first force convert to ASCII with [asciionly]. Because this can happen before the removal of characters via [replace_non_letters_non_numbers_with_whitespace], it can affect whitespace.
/// ```
/// # use fuzzywuzzy::fuzzywuzzy_compatible::utils::full_process;
/// assert_eq!(full_process("ABC What! do_ you mean? ... ", false), "abc what  do_ you mean");
/// // U+00E4
/// assert_eq!(full_process(" äbc ", false), "äbc");
/// assert_eq!(full_process(" äbc ", true), "bc");
/// // U+0061 + U+0308
/// // Notice the change in whitespace.
/// // This could also happen with various unicode symbols or punctuation.
/// assert_eq!(full_process(" a\u{0308}bc ", false), "a bc");
/// assert_eq!(full_process(" a\u{0308}bc ", true), "abc");
/// ```
pub fn full_process(s: &str, force_ascii: bool) -> String {
    let tmp = if force_ascii { asciionly(s) } else { s.into() };
    replace_non_letters_non_numbers_with_whitespace(&tmp)
        .to_lowercase()
        .trim()
        .into()
}
