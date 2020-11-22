//! Fuzzy string matching scoring primitives.

use std::collections::HashSet;
use utils;

/// Returns the ratio of the length of matching character sequences to the sum of the length of the input strings.
///
/// Take, for example, `"cd"` and `"abcd"`.
/// The matching sequence is `"cd"` with a length of 2.
///
/// It is present in both strings, so we count it twice for a total length of matching character sequences of 4.
///
/// The sum of the length of the input strings is `"abcd".len() (4) + "cd".len() (2) = 6`.
///
/// Therefore the returned value is `(4f32/6f32).round() = 67`
///
/// ```
/// # use fuzzywuzzy::fuzz::ratio;
/// assert_eq!(ratio("", ""), 100);
/// assert_eq!(ratio("", "nonempty"), 0);
/// assert_eq!(ratio("cd", "abcd"), 67);
/// assert_eq!(ratio("new york mets", "new york mets"), 100);
/// assert_eq!(ratio("new york mets", "new YORK mets"), 69);
/// assert_eq!(ratio("hello test", "hello world"), 57);
/// ```
pub fn ratio(a: &str, b: &str) -> u8 {
    check_trivial!(a, b);
    let matches: usize = utils::get_matching_blocks(a, b)
        .iter()
        .map(|&(_, _, s)| s)
        .sum();
    let sumlength: f32 = (a.chars().count() + b.chars().count()) as f32;
    if sumlength > 0.0 {
        (100.0 * (2.0 * (matches as f32) / sumlength)).round() as u8
    } else {
        100
    }
}

/// Return the ratio of the most similar substring as a number between 0 and 100.
///
/// The most similar substring is determined by finding the "optimal" alignment
/// of the two strings. Then the similarity ratio of the aligned strings is returned.
///
/// Note: in compatibility with fuzzywuzzy, a suboptimal sequence alignment
/// algorithm is used. In future versions, this may change.
///
/// ```
/// # use fuzzywuzzy::fuzz::partial_ratio;
/// assert_eq!(partial_ratio("", ""), 100);
/// assert_eq!(partial_ratio("", "nonempty"), 0);
/// assert_eq!(partial_ratio("ab", "abcd"), 100);
/// assert_eq!(partial_ratio("bc", "abcd"), 100);
/// assert_eq!(partial_ratio("cd", "abcd"), 100);
/// assert_eq!(partial_ratio("ad", "abcd"), 50);
/// assert_eq!(partial_ratio("ac", "abcd"), 50);
/// assert_eq!(partial_ratio("hello", "hello world"), 100);
/// assert_eq!(partial_ratio("new york mets", "the new york mets"), 100);
/// assert_eq!(partial_ratio("the new york mets", "new york mets"), 100);
/// // Note the order dependence due to not finding the optimal alignment
/// assert_eq!(partial_ratio(
///    "what about supercalifragilisticexpialidocious",
///    "supercalifragilisticexpialidocious about what"), 76);
/// assert_eq!(partial_ratio(
///    "supercalifragilisticexpialidocious about what",
///    "what about supercalifragilisticexpialidocious"), 86);
/// ```
pub fn partial_ratio(s1: &str, s2: &str) -> u8 {
    check_trivial!(s1, s2);
    let (shorter, longer) = if s1.chars().count() <= s2.chars().count() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let blocks = utils::get_matching_blocks(shorter, longer);
    let mut max: u8 = 0;
    for (i, j, _) in blocks {
        let long_start = if j > i { j - i } else { 0 };
        let long_end = std::cmp::min(long_start + shorter.chars().count(), longer.chars().count());
        let long_substr = &longer[long_start..long_end];
        let r = ratio(shorter, long_substr);
        if r > 99 {
            return 100;
        } else if r > max {
            max = r;
        }
    }
    max
}

/// Return a cleaned string with token sorted.
fn process_and_sort(s: &str, force_ascii: bool, full_process: bool) -> String {
    let ts = if full_process {
        utils::full_process(s, force_ascii)
    } else {
        s.to_string()
    };
    let mut ts_split: Vec<_> = ts.split_whitespace().collect();
    ts_split.sort();
    ts_split.join(" ")
}

/// Sorted Token
/// # find all alphanumeric tokens in the string
/// # sort those tokens and take ratio of resulting joined strings
/// # controls for unordered string elements
fn token_sort(s1: &str, s2: &str, partial: bool, force_ascii: bool, full_process: bool) -> u8 {
    check_trivial!(s1, s2);
    let sorted1 = process_and_sort(s1, force_ascii, full_process);
    let sorted2 = process_and_sort(s2, force_ascii, full_process);
    if partial {
        partial_ratio(sorted1.as_ref(), sorted2.as_ref())
    } else {
        ratio(sorted1.as_ref(), sorted2.as_ref())
    }
}

/// Return a measure of the sequences' similarity between 0 and 100, but sort the token before
/// comparing.
///
/// This essentially sorts 'words' in each string and then runs `ratio`.
///
/// By default, force_ascii and full_process should be true.
///
/// ```
/// # use fuzzywuzzy::fuzz::token_sort_ratio;
/// assert_eq!(token_sort_ratio("hello world", "world hello", true, true), 100);
/// assert_eq!(token_sort_ratio("new york mets", "the new york mets", true, true), 87);
/// assert_eq!(token_sort_ratio("new york mets", "new YORK mets", true, true), 100);
/// assert_eq!(token_sort_ratio(
///    "what about supercalifragilisticexpialidocious",
///    "supercalifragilisticexpialidocious about what", true, true), 100);
/// ```
pub fn token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to token_sort which checks.
    token_sort(s1, s2, false, force_ascii, full_process)
}

/// Return the ratio of the most similar substring as a number between 0 and 100, but sort the tokens
/// before comparing.
///
/// This essentially sorts 'words' in each string and then runs `partial_ratio`.
///
/// By default, force_ascii and full_process should be true.
///
/// ```
/// # use fuzzywuzzy::fuzz::partial_token_sort_ratio;
/// assert_eq!(partial_token_sort_ratio("hello world", "world hello", true, true), 100);
/// assert_eq!(partial_token_sort_ratio("new york mets", "the new york mets", true, true), 69);
/// assert_eq!(partial_token_sort_ratio("new york mets", "new YORK mets", true, true), 100);
/// assert_eq!(partial_token_sort_ratio("new york mets vs atlanta braves", "atlanta braves vs new york mets", true, true), 100);
/// assert_eq!(partial_token_sort_ratio(
///    "what about supercalifragilisticexpialidocious",
///    "supercalifragilisticexpialidocious about what", true, true), 100);
/// ```
pub fn partial_token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to token_sort which checks.
    token_sort(s1, s2, true, force_ascii, full_process)
}

/// Find all alphanumeric tokens in each string...
///  # treat them as a set
///  # construct two strings of the form: <sorted_intersection><sorted_remainder>
///  # take ratios of those two strings
///  # controls for unordered partial matches
fn token_set(s1: &str, s2: &str, partial: bool, force_ascii: bool, full_process: bool) -> u8 {
    check_trivial!(s1, s2);
    let (p1, p2) = if full_process {
        (
            utils::full_process(s1, force_ascii),
            utils::full_process(s2, force_ascii),
        )
    } else {
        (s1.to_string(), s2.to_string())
    };
    let t1: HashSet<_> = p1.split_whitespace().collect();
    let t2: HashSet<_> = p2.split_whitespace().collect();
    let mut intersection: Vec<_> = t1.intersection(&t2).cloned().collect();
    let mut diff1to2: Vec<_> = t1.difference(&t2).cloned().collect();
    let mut diff2to1: Vec<_> = t2.difference(&t1).cloned().collect();
    intersection.sort();
    diff1to2.sort();
    diff2to1.sort();
    let intersect_str = intersection.join(" ");
    let diff1to2_str = diff1to2.join(" ");
    let diff2to1_str = diff2to1.join(" ");
    let combined_1to2 = if !diff1to2_str.is_empty() {
        intersect_str.to_string() + " " + &diff1to2_str
    } else {
        intersect_str.to_string()
    };
    let combined_2to1 = if !diff2to1_str.is_empty() {
        intersect_str.to_string() + " " + &diff2to1_str
    } else {
        intersect_str.to_string()
    };
    if partial {
        *vec![
            partial_ratio(&intersect_str, &combined_1to2),
            partial_ratio(&intersect_str, &combined_2to1),
            partial_ratio(&combined_1to2, &combined_2to1),
        ]
        .iter()
        .max()
        .unwrap()
    } else {
        *vec![
            ratio(&intersect_str, &combined_1to2),
            ratio(&intersect_str, &combined_2to1),
            ratio(&combined_1to2, &combined_2to1),
        ]
        .iter()
        .max()
        .unwrap()
    }
}

/// Return the ratio of the most similar substring constructed from the strings treated as sets, as a number between 0 and 100.
///
/// Creates three sets from the two strings:
///  1. a sorted set containing words in both strings, joined by spaces
///  2. a sorted set containing words in both strings followed by words only in the first string, joined by spaces
///  3. a sorted set containing words in both strings followed by words only in the second string, joined by spaces
///
/// Note: the set of words in the intersection and difference are sorted before being concatenated and joined by spaces.
///
/// The ratio is computed between all three, pairwise, and the maximum is returned.
///
/// ```
/// # use fuzzywuzzy::fuzz::token_set_ratio;
/// assert_eq!(token_set_ratio("hello world", "world hello", true, true), 100);
/// assert_eq!(token_set_ratio("new york mets", "the new york mets", true, true), 100);
/// assert_eq!(token_set_ratio("new york mets", "new YORK mets", true, true), 100);
/// assert_eq!(token_set_ratio("new york mets vs atlanta braves", "atlanta braves vs new york mets", true, true), 100);
/// assert_eq!(token_set_ratio(
///    "what about supercalifragilisticexpialidocious",
///    "supercalifragilisticexpialidocious about what", true, true), 100);
/// ```
pub fn token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to token_set which checks.
    token_set(s1, s2, false, force_ascii, full_process)
}

/// Return the partial ratio of the most similar substring constructed from the strings treated as sets, as a number between 0 and 100.
///
/// Creates three sets from the two strings:
///  1. a sorted set containing words in both strings, joined by spaces
///  2. a sorted set containing words in both strings followed by words only in the first string, joined by spaces
///  3. a sorted set containing words in both strings followed by words only in the second string, joined by spaces
///
/// Note: the set of words in the intersection and difference are sorted before being concatenated and joined by spaces.
///
/// The partial ratio is computed between all three, pairwise, and the maximum is returned.
///
/// ```
/// # use fuzzywuzzy::fuzz::partial_token_set_ratio;
/// assert_eq!(partial_token_set_ratio("hello world", "world hello", true, true), 100);
/// assert_eq!(partial_token_set_ratio("new york mets", "the new york mets", true, true), 100);
/// assert_eq!(partial_token_set_ratio("new york mets", "new YORK mets", true, true), 100);
/// assert_eq!(partial_token_set_ratio("new york mets - atlanta braves", "atlanta braves - new york city mets", true, true), 100);
/// assert_eq!(partial_token_set_ratio(
///    "what about supercalifragilisticexpialidocious",
///    "supercalifragilisticexpialidocious about what", true, true), 100);
/// ```
pub fn partial_token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to token_set which checks.
    token_set(s1, s2, true, force_ascii, full_process)
}

/// Quick `ratio` comparison between two strings.
///
//  Runs utils::full_process on both strings.
//  Short circuits if either of the strings is empty after processing.
pub fn qratio(s1: &str, s2: &str, force_ascii: bool) -> u8 {
    check_trivial!(s1, s2);
    let (p1, p2) = (
        utils::full_process(s1, force_ascii),
        utils::full_process(s2, force_ascii),
    );
    if !utils::validate_string(p1.as_str()) || !utils::validate_string(p2.as_str()) {
        return 0;
    }
    ratio(&p1, &p2)
}

/// micro-quick-ratio: `qratio` comparison between two strings without forcing to ascii.
pub fn uqratio(s1: &str, s2: &str) -> u8 {
    // trivial check omitted because this is a shallow delegator to qratio which checks.
    qratio(s1, s2, false)
}

/// Return a measure of the sequences' similarity between 0 and 100, using a composite algorithm.
///
/// *Steps in the order they occur*
///  1. Run full_process from utils on both strings
///  2. Short circuit if this makes either string empty
///  3. Take the ratio of the two processed strings (`fuzz::ratio`)
///  4. Run checks to compare the length of the strings
///     * If one of the strings is more than 1.5 times as long as the other
///       use partial_ratio comparisons - scale partial results by 0.9
///       (this makes sure only full results can return 100)
///     * If one of the strings is over 8 times as long as the other
///       instead scale by 0.6
///  5. Run the other ratio functions
///     * if using partial ratio functions call partial_ratio,
///       partial_token_sort_ratio and partial_token_set_ratio
///       scale all of these by the ratio based on length
///     * otherwise call token_sort_ratio and token_set_ratio
///     * all token based comparisons are scaled by 0.95
///       (on top of any partial scalars)
///  6. Take the highest value from these results
///     round it and return it as an integer.
///
/// TODO: function is hard-coded to use partial functions?
///
/// ```
/// # use fuzzywuzzy::fuzz::wratio;
/// assert_eq!(wratio("", "", true, true), 100);
/// assert_eq!(wratio("hello world", "hello world", true, true), 100);
/// assert_eq!(wratio("hello world", "world hello", true, true), 95);
/// assert_eq!(wratio("new york mets", "new YORK mets", true, true), 100);
/// assert_eq!(wratio("new york mets", "the wonderful new york mets", true, true), 90);
/// assert_eq!(wratio("new york mets vs atlanta braves", "atlanta braves vs new york mets", true, true), 95);
/// ```
pub fn wratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    check_trivial!(s1, s2);
    let (p1, p2) = if full_process {
        (
            utils::full_process(s1, force_ascii),
            utils::full_process(s2, force_ascii),
        )
    } else {
        (s1.to_string(), s2.to_string())
    };
    let (p1r, p2r) = (p1.as_str(), p2.as_str());
    if !utils::validate_string(p1r) || !utils::validate_string(p2r) {
        return 0;
    }
    let mut try_partial = true;
    const UNBASE_SCALE: f64 = 0.95;
    let mut partial_scale = 0.90;

    let base = ratio(p1r, p2r);
    let (p1_len, p2_len) = (p1.chars().count(), p2.chars().count());
    let len_ratio = std::cmp::max(p1_len, p2_len) as f64 / std::cmp::min(p1_len, p2_len) as f64;

    // if strings are similar length, don't use partials
    if len_ratio < 1.5 {
        try_partial = false;
    }

    // if one string is much shorter than the other
    if len_ratio > 8.0 {
        partial_scale = 0.6;
    }

    if try_partial {
        let partial = partial_ratio(p1r, p2r) as f64 * partial_scale;
        let ptsor =
            partial_token_sort_ratio(p1r, p2r, true, false) as f64 * UNBASE_SCALE * partial_scale;
        let ptser =
            partial_token_set_ratio(p1r, p2r, true, false) as f64 * UNBASE_SCALE * partial_scale;
        // This conversion to u8 from the maximum f64 seems spooky, but let's hope nothing bad happens!
        return vec![base as f64, partial, ptsor, ptser]
            .iter()
            .cloned()
            .fold(f64::NAN, f64::max)
            .round() as u8;
    }
    let tsor = token_sort_ratio(p1r, p2r, true, false) as f64 * UNBASE_SCALE;
    let tser = token_set_ratio(p1r, p2r, true, false) as f64 * UNBASE_SCALE;
    vec![base as f64, tsor, tser]
        .iter()
        .cloned()
        .fold(f64::NAN, f64::max)
        .round() as u8
}

/// Runs `wratio` without forcing to ascii.
pub fn uwratio(s1: &str, s2: &str, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to wratio which checks.
    wratio(s1, s2, false, full_process)
}
