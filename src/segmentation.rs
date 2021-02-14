//! Segmenter trait and default implementations.
//!
//! Segmentation is how strings are split into tokens for comparison.
//! For example, two strings that *appear* identical might have different byte-level representations.
//!
//! Take `ä` and `ä`. Visually, these should be identical. However, the former
//! is Unicode character [ä (U+00E4)](https://www.compart.com/en/unicode/U+00E4)
//! while the latter is two adjacent Unicode characters [a (U+0061)](https://www.compart.com/en/unicode/U+0061)
//! and [◌̈ (U+0308)](https://www.compart.com/en/unicode/U+0308).
//!
//! Depending on the [Segmenter] used, comparing these strings will return different results.
//!
//! ```
//! # use fuzzywuzzy::segmentation::{Segmenter, ByteSegmenter};
//! // U+00E4
//! assert_eq!(ByteSegmenter.segment("ä"), vec![0xc3u8, 0xa4u8]);
//! // U+0061 + U+0308
//! assert_eq!(ByteSegmenter.segment("ä"), vec![0x61u8, 0xccu8, 0x88u8]);
//! ```
//! Given this segmentation, we would expect a comparison to return 0% similarity because every byte is different!
//!
//! However, even with more advanced segmentation strategies like [code point segmentation](CodePointSegmenter)
//! or [grapheme segmentation](GraphemeSegmenter), these will still have 0% similarity by any comparison algorithm.
//!
//! ```
//! # use fuzzywuzzy::segmentation::{Segmenter, CodePointSegmenter, GraphemeSegmenter};
//! // U+00E4
//! assert_eq!(CodePointSegmenter.segment("ä"), vec!['ä']);
//! assert_eq!(GraphemeSegmenter.segment("ä"), vec!["ä"]);
//! // U+0061 + U+0308
//! assert_eq!(CodePointSegmenter.segment("ä"), vec!['a', '\u{0308}']);
//! assert_eq!(GraphemeSegmenter.segment("ä"), vec!["ä"]);
//! // U+00E4 vs. U+0061 + U+0308
//! assert_ne!(GraphemeSegmenter.segment("ä"), GraphemeSegmenter.segment("ä"));
//! ```
//!
//! In order to usefully compare strings like these, [normalization][crate::normalization] must be done prior to segmentation.

/// Represents a strategy for segmenting a string into units for comparison.
///
/// The trait is also implemented for functions matching the signature of the `segment` method.
pub trait Segmenter<'a> {
    /// The type of the unit of comparison this strategy operates on.
    type Output: 'a + Eq;
    /// Produces units of comparison from a string according to the segmentation strategy.
    fn segment(&self, s: &'a str) -> Vec<Self::Output>;
}

impl<'a, F: Fn(&str) -> Vec<T>, T: 'a + Eq> Segmenter<'a> for F {
    type Output = T;
    fn segment(&self, s: &'a str) -> Vec<Self::Output> {
        self(s)
    }
}

/// A strategy for segmenting strings into their constituent bytes.
pub struct ByteSegmenter;

impl<'a> Segmenter<'a> for ByteSegmenter {
    // Returns an owned `Vec<u8>` because allocating additional `u8`s is cheaper than pointers into the original string.
    type Output = u8;
    fn segment(&self, s: &'a str) -> Vec<Self::Output> {
        s.as_bytes().iter().map(|x| *x).collect()
    }
}

/// A strategy for segmenting strings into their constituent Unicode code points.
///
/// Internally, this is just `s.chars().collect()`.
///
/// Note that `char` is a Unicode Scalar Value which is a subset of Unicode code points disallowing surrogates.
/// UTF-8, which all Rust strings are guaranteed to be, also disallows surrogates.
/// So all of the Unicode Scalar Values produced here are UTF-8 code points.
pub struct CodePointSegmenter;

impl<'a> Segmenter<'a> for CodePointSegmenter {
    type Output = char;
    fn segment(&self, s: &'a str) -> Vec<Self::Output> {
        s.chars().collect()
    }
}

#[cfg(feature = "segmentation")]
pub use self::unicode_segmenters::*;

#[cfg(feature = "segmentation")]
mod unicode_segmenters {
    use super::Segmenter;
    use unicode_segmentation::UnicodeSegmentation;

    /// A strategy for segmenting strings into their constituent Unicode graphemes. Requires default feature "segmentation".
    ///
    /// This just delegates to [unicode_segmentation].
    pub struct GraphemeSegmenter;

    impl<'a> Segmenter<'a> for GraphemeSegmenter {
        type Output = &'a str;
        fn segment(&self, s: &'a str) -> Vec<Self::Output> {
            s.graphemes(true).collect()
        }
    }
}
