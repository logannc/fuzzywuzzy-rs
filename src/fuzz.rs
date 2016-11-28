use std::collections::HashSet;
use std::cmp::Ordering::Equal;
use string_matcher;
use utils;

pub fn ratio(s1: &str, s2: &str) -> u8 {
    let m = string_matcher::SequenceMatcher::new(s1, s2);
    (m.ratio() * 100.0) as u8
}

pub fn partial_ratio(s1: &str, s2: &str) -> u8 {
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1.to_string(), s2.to_string())
    } else {
        (s2.to_string(), s1.to_string())
    };
    let m = string_matcher::SequenceMatcher::new(&shorter, &longer);
    let blocks = m.get_matching_blocks();
    let mut scores: Vec<f32> = Vec::new();
    for (idx_1, idx_2, len) in blocks {
        let long_start = if idx_2 - idx_1 > 0 {
            idx_2 - idx_1
        } else {
            0
        };
        let long_end = long_start + shorter.len();
        let long_substr = &longer[long_start..long_end];
        let m2 = string_matcher::SequenceMatcher::new(&shorter, long_substr);
        let r = m2.ratio();
        if r > 0.995 {
            return 100
        } else {
            scores.push(r.clone())
        }
    }
    (scores.iter().max_by(|a,b| a.partial_cmp(b).unwrap_or(Equal)).unwrap_or(&0.0) * 100.0) as u8
}

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

fn token_sort(s1: &str, s2: &str, partial: bool, force_ascii: bool, full_process: bool) -> u8 {
    let sorted1 = process_and_sort(s1, force_ascii, full_process);
    let sorted2 = process_and_sort(s2, force_ascii, full_process);
    if partial {
        partial_ratio(sorted1.as_ref(), sorted2.as_ref())
    } else {
        ratio(sorted1.as_ref(), sorted2.as_ref())
    }
}

pub fn token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_sort(s1, s2, false, force_ascii, full_process)
}

pub fn partial_token_sort_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_sort(s1, s2, true, force_ascii, full_process)
}

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
        vec![
            partial_ratio(&intersect_str, &combined_1to2),
            partial_ratio(&intersect_str, &combined_2to1),
            partial_ratio(&combined_1to2, &combined_2to1)
        ].iter().max().unwrap().clone()
    } else {
        vec![
            ratio(&intersect_str, &combined_1to2),
            ratio(&intersect_str, &combined_2to1),
            ratio(&combined_1to2, &combined_2to1)
        ].iter().max().unwrap().clone()
    }
}

pub fn token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_set(s1, s2, false, force_ascii, full_process)
}

pub fn partial_token_set_ratio(s1: &str, s2: &str, force_ascii: bool, full_process: bool) -> u8 {
    token_set(s1, s2, true, force_ascii, full_process)
}

pub fn QRatio(s1: &str, s2: &str, force_ascii: bool) -> u8 {
    let (p1, p2) = (utils::full_process(s1, force_ascii), utils::full_process(s2, force_ascii));
    ratio(&p1, &p2)
}

pub fn UQRatio(s1: &str, s2: &str, force_ascii: bool) -> u8 {
    QRatio(s1, s2, false)
}