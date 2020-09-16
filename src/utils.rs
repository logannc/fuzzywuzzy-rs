/// Process string by
/// # removing all but letters and numbers
/// # trim whitespace
/// # force to lower case
///
/// If force_ascii == true, force convert to ascii. By default, this is false.
pub fn full_process(s: &str, force_ascii: bool) -> String {
    let mut result = s.to_string();
    if force_ascii {
        result = result.chars().filter(char::is_ascii).collect();
    }
    result = result
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();
    result.make_ascii_lowercase();
    result.trim().to_string()
}

/// Ensures that the input string is non-empty.
pub fn validate_string(s: &str) -> bool {
    !s.is_empty()
}

fn find_longest_match<'a>(
    shorter: &'a str,
    longer: &'a str,
    low1: usize,
    high1: usize,
    low2: usize,
    high2: usize,
) -> (usize, usize, usize) {
    // https://github.com/python-git/python/blob/master/Lib/difflib.py#L351
    // algo:
    //  In other words, of all maximal matching blocks, return one that
    //  starts earliest in a, and of all those maximal matching blocks that
    //  start earliest in a, return the one that starts earliest in b.

    // In MY words:
    // So, try to find a block of size shorter.len(), else decrement size.
    // for each block size, start from the front of a and return if only one match
    // If multiple matches for a given block size and index, return the one that starts
    // earliest in b.
    let longsub = &longer[low2..high2];
    let slen = high1 - low1;
    for size in (1..slen + 1).rev() {
        for start in 0..slen - size + 1 {
            let substr = &shorter[low1 + start..low1 + start + size];
            let matches: Vec<(usize, &'a str)> = longsub.match_indices(substr).collect();
            // Does this need to be sorted?
            if let Some(&(startb, matchstr)) = matches.first() {
                return (low1 + start, low2 + startb, matchstr.len());
            }
        }
    }
    (low1, low2, 0)
}

pub fn get_matching_blocks<'a>(shorter: &'a str, longer: &'a str) -> Vec<(usize, usize, usize)> {
    // https://github.com/python-git/python/blob/master/Lib/difflib.py#L461
    let (len1, len2) = (shorter.len(), longer.len());
    let mut queue: Vec<(usize, usize, usize, usize)> = vec![(0, len1, 0, len2)];
    let mut matching_blocks = Vec::new();
    while let Some((low1, high1, low2, high2)) = queue.pop() {
        let (i, j, k) = find_longest_match(shorter, longer, low1, high1, low2, high2);
        if k != 0 {
            matching_blocks.push((i, j, k));
            if low1 < i && low2 < j {
                queue.push((low1, i, low2, j));
            }
            if i + k < high1 && j + k < high2 {
                queue.push((i + k, high1, j + k, high2));
            }
        }
    }
    matching_blocks.sort(); // Is this necessary?
    let (mut i1, mut j1, mut k1) = (0, 0, 0);
    let mut non_adjacent = Vec::new();
    for (i2, j2, k2) in matching_blocks {
        if i1 + k1 == i2 && j1 + k1 == j2 {
            k1 += k2;
        } else {
            if k1 != 0 {
                non_adjacent.push((i1, j1, k1));
            }
            i1 = i2;
            j1 = j2;
            k1 = k2;
        }
    }
    if k1 != 0 {
        non_adjacent.push((i1, j1, k1));
    }
    non_adjacent.push((len1, len2, 0));
    non_adjacent
}
