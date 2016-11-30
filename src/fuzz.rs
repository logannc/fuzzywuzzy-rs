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

pub fn partial_ratio(s1: &str, s2: &str) -> u8 {
    let (shorter, longer) = if s1.len() <= s2.len() {
        (s1.to_string(), s2.to_string())
    } else {
        (s2.to_string(), s1.to_string())
    };
    let blocks = utils::get_matching_blocks(&shorter, &longer);
    let mut max: u8 = 0;
    for (i, _, k) in blocks {
        let substr = &shorter[i..i + k];
        let r = ratio(&shorter, substr);
        if r > 99 {
            return 100;
        } else if r > max {
            max = r;
        }
    }
    max
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

pub fn qratio(s1: &str, s2: &str, force_ascii: bool) -> u8 {
    let (p1, p2) = (utils::full_process(s1, force_ascii), utils::full_process(s2, force_ascii));
    ratio(&p1, &p2)
}

pub fn uqratio(s1: &str, s2: &str) -> u8 {
    qratio(s1, s2, false)
}