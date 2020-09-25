use std::collections::HashSet;
use utils;

pub fn ratio(s1: &str, s2: &str) -> u8 {
    check_trivial!(s1, s2);
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let matches: usize = utils::get_matching_blocks(shorter, longer)
        .iter()
        .map(|&(_, _, s)| s)
        .sum();
    let sumlength: f32 = (s1.len() + s2.len()) as f32;
    if sumlength > 0.0 {
        (100.0 * (2.0 * (matches as f32) / sumlength)).round() as u8
    } else {
        100
    }
}

/// Return the ratio of the most similar substring as a number between 0 and 100.
pub fn partial_ratio(s1: &str, s2: &str) -> u8 {
    check_trivial!(s1, s2);
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1.to_string(), s2.to_string())
    } else {
        (s2.to_string(), s1.to_string())
    };
    let blocks = utils::get_matching_blocks(&shorter, &longer);
    let mut max: u8 = 0;
    for (i, j, _) in blocks {
        let long_start = if j > i { j - i } else { 0 };
        let long_end = std::cmp::min(long_start + shorter.len(), longer.len());
        let long_substr = &longer[long_start..long_end];
        let r = ratio(&shorter, long_substr);
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
/// By default, force_ascii and full_process should be true.
pub fn token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to token_sort which checks.
    token_sort(s1, s2, false, force_ascii, full_process)
}

/// Return the ratio of the most similar substring as a number between 0 and 100, but sort the token
/// before comparing.
///
/// By default, force_ascii and full_process should be true.
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
        intersect_str.to_string() + &diff1to2_str
    } else {
        intersect_str.to_string()
    };
    let combined_2to1 = if !diff2to1_str.is_empty() {
        intersect_str.to_string() + &diff2to1_str
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

pub fn token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to token_set which checks.
    token_set(s1, s2, false, force_ascii, full_process)
}

pub fn partial_token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to token_set which checks.
    token_set(s1, s2, true, force_ascii, full_process)
}

/// Quick ratio comparison between two strings.
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

pub fn uqratio(s1: &str, s2: &str) -> u8 {
    // trivial check omitted because this is a shallow delegator to qratio which checks.
    qratio(s1, s2, false)
}

/// Return a measure of the sequences' similarity between 0 and 100, using different algorithms.
///
/// ** Steps in the order they occur **
/// #. Run full_process from utils on both strings
/// #. Short circuit if this makes either string empty
/// #. Take the ratio of the two processed strings (fuzz.ratio)
/// #. Run checks to compare the length of the strings
///     * If one of the strings is more than 1.5 times as long as the other
///       use partial_ratio comparisons - scale partial results by 0.9
///       (this makes sure only full results can return 100)
///     * If one of the strings is over 8 times as long as the other
///       instead scale by 0.6
/// #. Run the other ratio functions
///     * if using partial ratio functions call partial_ratio,
///       partial_token_sort_ratio and partial_token_set_ratio
///       scale all of these by the ratio based on length
///     * otherwise call token_sort_ratio and token_set_ratio
///     * all token based comparisons are scaled by 0.95
///       (on top of any partial scalars)
/// #. Take the highest value from these results
///    round it and return it as an integer.
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
    let len_ratio =
        std::cmp::max(p1.len(), p2.len()) as f64 / std::cmp::min(p1.len(), p2.len()) as f64;

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

pub fn uwratio(s1: &str, s2: &str, full_process: bool) -> u8 {
    // trivial check omitted because this is a shallow delegator to wratio which checks.
    wratio(s1, s2, false, full_process)
}

#[cfg(test)]
mod tests {
    use fuzz;
    use utils;

    struct Fixture {
        s1: &'static str,
        s1a: &'static str,
        s2: &'static str,
        s3: &'static str,
        s4: &'static str,
        s5: &'static str,
        s6: &'static str,
        s7: &'static str,
        s8: &'static str,
        s8a: &'static str,
        s9: &'static str,
        s9a: &'static str,
        s10: &'static str,
        s10a: &'static str,
        // TODO: Test silly corner cases,
        cirque_strings: &'static [&'static str; 6],
        baseball_strings: &'static [&'static str; 4],
    }

    impl Fixture {
        pub fn new() -> Self {
            Self {
                s1: "new york mets",
                s1a: "new york mets",
                s2: "new YORK mets",
                s3: "the wonderful new york mets",
                s4: "new york mets vs atlanta braves",
                s5: "atlanta braves vs new york mets",
                s6: "new york mets - atlanta braves",
                s7: "new york city mets - atlanta braves",
                s8: "{",
                s8a: "{",
                s9: "{a",
                s9a: "{a",
                s10: "a{",
                s10a: "{b",
                cirque_strings: &[
                    "cirque du soleil - zarkana - las vegas",
                    "cirque du soleil ",
                    "cirque du soleil las vegas",
                    "zarkana las vegas",
                    "las vegas cirque du soleil at the bellagio",
                    "zarakana - cirque du soleil - bellagio",
                ],
                baseball_strings: &[
                    "new york mets vs chicago cubs",
                    "chicago cubs vs chicago white sox",
                    "philladelphia phillies vs atlanta braves",
                    "braves vs mets",
                ],
            }
        }
    }

    #[test]
    fn test_equal() {
        let f = Fixture::new();
        assert_eq!(fuzz::ratio(f.s1, f.s1a), 100);
        assert_eq!(fuzz::ratio(f.s8, f.s8a), 100);
        assert_eq!(fuzz::ratio(f.s9, f.s9a), 100);
        // TODO: These are from the Logan's tests, so testing these scores may not be valid.
        assert_eq!(fuzz::ratio("hello test", "hello world"), 57);
        assert_eq!(fuzz::ratio("hello test", "hello worlasdfasd"), 52);
    }

    #[test]
    fn test_case_insensitive() {
        let f = Fixture::new();
        assert_ne!(fuzz::ratio(f.s1, f.s2), 100);
        assert_eq!(
            fuzz::ratio(
                utils::full_process(f.s1, false).as_str(),
                utils::full_process(f.s2, false).as_str()
            ),
            100
        );
    }

    #[test]
    fn test_partial_ratio() {
        let f = Fixture::new();
        assert_eq!(fuzz::partial_ratio(f.s1, f.s3), 100);
        // TODO: These are from the Logan's tests, so testing these scores may not be valid.
        assert_eq!(fuzz::partial_ratio("hello", "hello world"), 100);
    }

    #[test]
    fn test_token_sort_ratio() {
        let f = Fixture::new();
        assert_eq!(fuzz::token_sort_ratio(f.s1, f.s1a, true, true), 100);
        // TODO: These are from the Logan's tests, so testing these scores may not be valid.
        assert_eq!(
            fuzz::token_sort_ratio("hello world", "world hello", false, false),
            100
        );
    }

    #[test]
    fn test_partial_token_sort_ratio() {
        let f = Fixture::new();
        assert_eq!(fuzz::partial_token_sort_ratio(f.s1, f.s1a, true, true), 100);
        assert_eq!(fuzz::partial_token_sort_ratio(f.s4, f.s5, true, true), 100);
        assert_eq!(
            fuzz::partial_token_sort_ratio(f.s8, f.s8a, true, false),
            100
        );
        assert_eq!(fuzz::partial_token_sort_ratio(f.s9, f.s9a, true, true), 100);
        assert_eq!(
            fuzz::partial_token_sort_ratio(f.s9, f.s9a, true, false),
            100
        );
        assert_eq!(
            fuzz::partial_token_sort_ratio(f.s10, f.s10a, true, false),
            50
        );
    }

    #[test]
    fn test_token_set_ratio() {
        let f = Fixture::new();
        assert_eq!(fuzz::token_set_ratio(f.s4, f.s5, true, true), 100);
        assert_eq!(fuzz::token_set_ratio(f.s8, f.s8a, true, false), 100);
        assert_eq!(fuzz::token_set_ratio(f.s9, f.s9a, true, true), 100);
        assert_eq!(fuzz::token_set_ratio(f.s9, f.s9a, true, false), 100);
        assert_eq!(fuzz::token_set_ratio(f.s10, f.s10a, true, false), 50);
    }

    #[test]
    fn test_partial_token_set_ratio() {
        let f = Fixture::new();
        assert_eq!(fuzz::partial_token_set_ratio(f.s4, f.s7, true, true), 100);
    }

    #[test]
    fn test_wratio_equal() {
        let f = Fixture::new();
        assert_eq!(fuzz::wratio(f.s1, f.s1a, true, true), 100);
    }

    #[test]
    fn test_wratio_case_insensitive() {
        let f = Fixture::new();
        assert_eq!(fuzz::wratio(f.s1, f.s2, true, true), 100);
    }

    #[test]
    fn test_wratio_partial_match() {
        let f = Fixture::new();
        assert_eq!(fuzz::wratio(f.s1, f.s3, true, true), 90);
    }

    #[test]
    fn test_wratio_misordered_match() {
        let f = Fixture::new();
        assert_eq!(fuzz::wratio(f.s4, f.s5, true, true), 95);
    }

    #[test]
    fn test_empty_string_score_100() {
        let f = Fixture::new();
        assert_eq!(fuzz::ratio("", ""), 100);
        assert_eq!(fuzz::partial_ratio("", ""), 100);
    }
}
