use std::collections::HashSet;
use utils;

pub fn ratio(s1: &str, s2: &str) -> u8 {
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };
    let matches: usize =
        utils::get_matching_blocks(shorter, longer).iter().map(|&(_, _, s)| s).sum();
    let sumlength: f32 = (s1.len() + s2.len()) as f32;
    if sumlength > 0.0 {
        (100.0 * (2.0 * (matches as f32) / sumlength)).round() as u8
    } else {
        100
    }
}

/// Return the ratio of the most similar substring as a number between 0 and 100.
pub fn partial_ratio(s1: &str, s2: &str) -> u8 {
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1.to_string(), s2.to_string())
    } else {
        (s2.to_string(), s1.to_string())
    };
    let blocks = utils::get_matching_blocks(&shorter, &longer);
    let mut max: u8 = 0;
    for (i, j, _) in blocks {
        let long_start = if j > i {
            j - i
        } else {
            0
        };
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
pub fn token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_sort(s1, s2, false, force_ascii, full_process)
}

/// Return the ratio of the most similar substring as a number between 0 and 100, but sort the token
/// before comparing.
pub fn partial_token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_sort(s1, s2, true, force_ascii, full_process)
}

/// Find all alphanumeric tokens in each string...
///  # treat them as a set
///  # construct two strings of the form: <sorted_intersection><sorted_remainder>
///  # take ratios of those two strings
///  # controls for unordered partial matches
fn token_set(s1: &str, s2: &str, partial: bool, force_ascii: bool, full_process: bool) -> u8 {
    let (p1, p2) = if full_process {
        (utils::full_process(s1, force_ascii), utils::full_process(s2, force_ascii))
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
    let combined_1to2 = if diff1to2_str.len() > 0 {
        intersect_str.to_string() + &diff1to2_str
    } else {
        intersect_str.to_string()
    };
    let combined_2to1 = if diff2to1_str.len() > 0 {
        intersect_str.to_string() + &diff2to1_str
    } else {
        intersect_str.to_string()
    };
    if partial {
        vec![partial_ratio(&intersect_str, &combined_1to2),
             partial_ratio(&intersect_str, &combined_2to1),
             partial_ratio(&combined_1to2, &combined_2to1)]
            .iter()
            .max()
            .unwrap()
            .clone()
    } else {
        vec![ratio(&intersect_str, &combined_1to2),
             ratio(&intersect_str, &combined_2to1),
             ratio(&combined_1to2, &combined_2to1)]
            .iter()
            .max()
            .unwrap()
            .clone()
    }
}

pub fn token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_set(s1, s2, false, force_ascii, full_process)
}

pub fn partial_token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_set(s1, s2, true, force_ascii, full_process)
}

/// Quick ratio comparison between two strings.
///
//  Runs utils::full_process on both strings.
//  Short circuits if either of the strings is empty after processing.
pub fn qratio(s1: &str, s2: &str, force_ascii: bool) -> u8 {
    let (p1, p2) = (utils::full_process(s1, force_ascii), utils::full_process(s2, force_ascii));
    if !utils::validate_string(p1.as_str()) || !utils::validate_string(p2.as_str()) {
        return 0;
    }
    ratio(&p1, &p2)
}

pub fn uqratio(s1: &str, s2: &str) -> u8 {
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
    let (p1, p2) = if full_process {
        (utils::full_process(s1, force_ascii), utils::full_process(s2, force_ascii))
    } else {
        (s1.to_string(), s2.to_string())
    };
    let (p1r, p2r) = (p1.as_str(), p2.as_str());
    if !utils::validate_string(p1r) || !utils::validate_string(p2r) {
        return 0;
    }
    let mut try_partial = true;
    let unbase_scale = 0.95;
    let mut partial_scale = 0.90;

    let base = ratio(p1r, p2r);
    let len_ratio = std::cmp::max(p1.len(), p2.len()) as f64 / std::cmp::min(p1.len(), p2.len()) as f64;

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
        let ptsor = partial_token_sort_ratio(p1r, p2r, true, false) as f64 * unbase_scale * partial_scale;
        let ptser = partial_token_set_ratio(p1r, p2r, true, false) as f64 * unbase_scale * partial_scale;
        // This conversion to u8 from the maximum f64 seems spooky, but let's hope nothing bad happens!
        return vec![base as f64, partial, ptsor, ptser].iter().cloned().fold(0./0., f64::max).round() as u8;
    }
    let tsor = token_sort_ratio(p1r, p2r, true, false) as f64 * unbase_scale;
    let tser = token_set_ratio(p1r, p2r, true, false) as f64 * unbase_scale;
    vec![base as f64, tsor, tser].iter().cloned().fold(0./0., f64::max).round() as u8
}

pub fn uwratio(s1: &str, s2: &str, full_process: bool) -> u8 {
    wratio(s1, s2, false, full_process)
}
