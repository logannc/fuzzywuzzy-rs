//! Standalone functions used by the rest of the crate. You might also find them useful.

/// Used to preprocess strings into 'canonical' forms.
///
/// Process string by
/// 1. if `force_ascii`, remove non-ascii characters
/// 2. replace all non-alphanumeric characters with a space
/// 3. force to lower case
/// 4. trim whitespace
///
/// ```
/// # use fuzzywuzzy::utils::full_process;
/// assert_eq!(full_process("Lorem Ipsum", false), "lorem ipsum");
/// assert_eq!(full_process("C'est la vie", false), "c est la vie");
/// assert_eq!(full_process("Ça va?", false), "ça va");
/// assert_eq!(full_process("Cães danados", false), "cães danados");
/// assert_eq!(full_process("¬Camarões assados", false), "camarões assados");
/// assert_eq!(full_process("a¬4ሴ2€耀", false), "a 4ሴ2 耀");
/// assert_eq!(full_process("Á", false), "á");
///
/// assert_eq!(full_process("Lorem Ipsum", true), "lorem ipsum");
/// assert_eq!(full_process("C'est la vie", true), "c est la vie");
/// assert_eq!(full_process("Ça va?", true), "a va");
/// assert_eq!(full_process("Cães danados", true), "ces danados");
/// assert_eq!(full_process("¬Camarões assados", true), "camares assados");
/// // Notice that the filtering of non-ascii values occurs *before* replacing
/// // non-alphanumeric with whitespace, which changes the result dramatically.
/// assert_eq!(full_process("a¬4ሴ2€耀", true), "a42");
/// assert_eq!(full_process("Á", true), "");
/// ```
pub fn full_process(s: &str, force_ascii: bool) -> String {
    let mut result = s.to_string();
    if force_ascii {
        result = result.chars().filter(char::is_ascii).collect();
    }
    result = result
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();
    result.to_lowercase().trim().into()
}

/// A vestigial function from the port from Python's fuzzywuzzy.
///
/// We, [`fuzzywuzzy-rs`](https://github.com/logannc/fuzzywuzzy-rs), attempt to maintain identical results with [`fuzzywuzzy-py`](https://github.com/seatgeek/fuzzywuzzy).
/// This function has been kept so that if the python version adds constraints, it is easy to propagate.
///
/// It makes sure the string is non-empty.
///
/// ```
/// # use fuzzywuzzy::utils::validate_string;
/// assert_eq!(validate_string(""), false);
/// assert_eq!(validate_string("anything else"), true);
/// ```
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

/// Returns list of triples describing matching sequences.
///
/// The first number is the index in the first string of the beginning of the match.
/// The second number is the index of the second string of the beginning of the match.
/// The final number is the length of the match.
///
/// The final matching sequence will be a trivial matching sequence of (a.len(), b.len(), 0) and will be the only match of length 0.
///
/// ```
/// # use fuzzywuzzy::utils::get_matching_blocks;
/// assert_eq!(get_matching_blocks("abxcd", "abcd"), vec![(0, 0, 2), (3, 2, 2), (5, 4, 0)]);
/// ```
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

/// some common short circuiting for ratio finding functions.
/// If the strings are equal, they have a ratio of 100%.
/// If only one of the strings is empty, they have a ratio of 0%.
macro_rules! check_trivial {
    ($s1:expr, $s2:expr) => {
        if $s1 == $s2 {
            return 100;
        }
        if $s1.is_empty() ^ $s2.is_empty() {
            return 0;
        }
    };
}
