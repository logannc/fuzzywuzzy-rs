//! Contains comparison primitives used to build up the rest of the library.

/// Returns list of triples describing matching sequences.
///
/// The first number is the index in the first string of the beginning of the match.
/// The second number is the index of the second string of the beginning of the match.
/// The final number is the length of the match.
///
/// The final matching sequence will be a trivial matching sequence of (a.len(),
/// b.len(), 0) and will be the only match of length 0.
///
/// ```
/// # use fuzzywuzzy::segmentation::{Segmenter, CodePointSegmenter, GraphemeSegmenter};
/// # use fuzzywuzzy::primitives::get_matching_blocks;
/// assert_eq!(get_matching_blocks(&CodePointSegmenter.segment("abxcd"), &CodePointSegmenter.segment("abcd")), vec![(0, 0, 2), (3, 2, 2), (5, 4, 0)]);
/// assert_eq!(get_matching_blocks(&CodePointSegmenter.segment("abcd"), &CodePointSegmenter.segment("abxcd")), vec![(0, 0, 2), (2, 3, 2), (4, 5, 0)]);
/// assert_eq!(get_matching_blocks(&CodePointSegmenter.segment("chance"), &CodePointSegmenter.segment("スマホでchance")), vec![(0, 4, 6), (6, 10, 0)]);
/// assert_eq!(get_matching_blocks(&GraphemeSegmenter.segment("chance"), &GraphemeSegmenter.segment("スマホでchance")), vec![(0, 4, 6), (6, 10, 0)]);
/// assert_eq!(get_matching_blocks(&CodePointSegmenter.segment("किमप"), &CodePointSegmenter.segment("किमपि")), vec![(0, 0, 4), (4, 5, 0)]);
/// assert_eq!(get_matching_blocks(&GraphemeSegmenter.segment("किमप"), &GraphemeSegmenter.segment("किमपि")), vec![(0, 0, 2), (3, 3, 0)]);
/// ```
#[allow(clippy::many_single_char_names)]
pub fn get_matching_blocks<T: Eq>(a: &[T], b: &[T]) -> Vec<(usize, usize, usize)> {
    let flipped;
    let (shorter, len1, longer, len2) = {
        let a_len = a.len();
        let b_len = b.len();
        if a_len <= b_len {
            flipped = false;
            (a, a_len, b, b_len)
        } else {
            flipped = true;
            (b, b_len, a, a_len)
        }
    };
    // https://github.com/python-git/python/blob/master/Lib/difflib.py#L461
    let mut queue = vec![(0, len1, 0, len2)];
    let mut matching_blocks = Vec::new();
    while let Some((low1, high1, low2, high2)) = queue.pop() {
        let (i, j, k) = find_longest_match(shorter, longer, low1, high1, low2, high2);
        debug_assert!(i <= shorter.len());
        debug_assert!(j <= longer.len());
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
    matching_blocks.sort_unstable();
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
        .into_iter()
        .map(|(i, j, k)| if flipped { (j, i, k) } else { (i, j, k) })
        .collect()
}

/// TODO: doc + tests
pub fn find_longest_match<T: Eq>(
    shorter: &[T],
    longer: &[T],
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
    //
    // In MY words: So, try to find a block of size `shorter.len()`[1], else
    // decrement size. For each block size, start from the front of `longer`
    // and return the earliest match for a given block size and index.
    //
    // [1] - because of the calling context, we actually use `high1 - low1`
    // for the length because we might be indexing into the middle of `shorter`
    debug_assert!(low1 <= high1);
    debug_assert!(low2 <= high2);
    debug_assert!(high1 <= shorter.len());
    debug_assert!(high2 <= longer.len());
    let longsub = &longer[low2..high2];
    let len = high1 - low1;
    for size in (1..len + 1).rev() {
        for start in 0..len - size + 1 {
            let shortsub = &shorter[low1 + start..low1 + start + size];
            for window_start in 0..((high2 - low2) - size + 1) {
                let window = &longsub[window_start..window_start + size];
                if window == shortsub {
                    return (low1 + start, low2 + window_start, size);
                }
            }
        }
    }
    (low1, low2, 0)
}
