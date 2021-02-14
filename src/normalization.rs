//! Normalizer trait and default implementations.
//!
//! Normalization is how strings are normalized into characters you want to
//! consider equal. For example, if you want a case insensitive comparison,
//! you might consider the [LowerCaseNormalizer].
//!
//! ```
//! # use fuzzywuzzy::normalization::{Normalizer, LowerCaseNormalizer, FormCNormalization, MultipleNormalizer};
//! assert_eq!(LowerCaseNormalizer.normalize("this STRING"), LowerCaseNormalizer.normalize("THIS string"));
//! let a1 = "ä"; // U+00E4
//! let a2 = "ä"; // U+0061 + U+0308
//! let a3 = "Ä"; // U+0041 + U+0308
//! assert_ne!(a1, a2);
//! assert_eq!(FormCNormalization.normalize(a2), a1);
//! let multiple_normalizers = MultipleNormalizer::with(vec![Box::new(LowerCaseNormalizer), Box::new(FormCNormalization)]);
//! assert_eq!(multiple_normalizers.normalize(a3), a1);
//! ```

/// Represents a strategy for normalizing string characters into a canonical value of their equivalence class.
///
/// i.e., in a case-insensitive context, 'a' might be the canonical value for the equivalence class of ASCII A's: `['a', 'A']`.
pub trait Normalizer {
    fn normalize(&self, s: &str) -> String;
}

impl<F: Fn(&str) -> String> Normalizer for F {
    fn normalize(&self, s: &str) -> String {
        self(s)
    }
}

/// Doesn't modify any characters. The Identity-transform [Normalizer].
pub struct PassthroughNormalizer;

impl Normalizer for PassthroughNormalizer {
    fn normalize(&self, s: &str) -> String {
        s.into()
    }
}

// ew, need a better name
/// Compose a sequence of [Normalizer]s together into one [Normalizer].
///
/// They are executed in order.
pub struct MultipleNormalizer {
    normalizers: Vec<Box<dyn Normalizer>>,
}

impl MultipleNormalizer {
    pub fn with(normalizers: Vec<Box<dyn Normalizer>>) -> MultipleNormalizer {
        MultipleNormalizer { normalizers }
    }
}

impl Normalizer for MultipleNormalizer {
    fn normalize(&self, s: &str) -> String {
        let mut current = s.to_owned();
        for normalizer in self.normalizers.iter() {
            current = normalizer.normalize(&current);
        }
        current
    }
}

/// Normalizes strings by lower-casing all letters.
pub struct LowerCaseNormalizer;

impl Normalizer for LowerCaseNormalizer {
    fn normalize(&self, s: &str) -> String {
        s.to_lowercase().into()
    }
}

/// Removes non-ASCII codepoints.
pub struct AsciiOnlyFilter;

impl Normalizer for AsciiOnlyFilter {
    fn normalize(&self, s: &str) -> String {
        s.chars().filter(char::is_ascii).collect()
    }
}

#[cfg(feature = "segmentation")]
pub use self::unicode_normalizers::*;

#[cfg(feature = "segmentation")]
mod unicode_normalizers {
    use super::Normalizer;
    use unicode_normalization::UnicodeNormalization;

    /// Performs Unicode Normalization Form C (canonical decomposition followed by canonical composition).
    ///
    /// This just delegates to [unicode_normalization].
    pub struct FormCNormalization;

    impl Normalizer for FormCNormalization {
        fn normalize(&self, s: &str) -> String {
            s.nfc().collect()
        }
    }

    /// Performs Unicode Normalization Form KC (compatibility decomposition followed by canonical composition).
    ///
    /// This just delegates to [unicode_normalization].
    pub struct FormKCNormalization;

    impl Normalizer for FormKCNormalization {
        fn normalize(&self, s: &str) -> String {
            s.nfkc().collect()
        }
    }

    /// Performs Unicode Normalization Form D (canonical decomposition).
    ///
    /// This just delegates to [unicode_normalization].
    /// ```
    /// # use fuzzywuzzy::normalization::{Normalizer, FormDNormalization};
    /// // FormDNormalization.normalize(U+00E4) == (U+0061 + U+0308)
    /// assert_eq!(FormDNormalization.normalize("ä"), "a\u{0308}");
    /// ```
    pub struct FormDNormalization;
    impl Normalizer for FormDNormalization {
        fn normalize(&self, s: &str) -> String {
            s.nfd().collect()
        }
    }

    /// Performs Unicode Normalization Form KD (compatibility decomposition).
    ///
    /// This just delegates to [unicode_normalization].
    pub struct FormKDNormalization;
    impl Normalizer for FormKDNormalization {
        fn normalize(&self, s: &str) -> String {
            s.nfkd().collect()
        }
    }

    /// Performs CJK Compatibility Ideograph-to-Standarized Variation Sequence normalization.
    ///
    /// This just delegates to [unicode_normalization]. "This is not part of the canonical or compatibility decomposition algorithms, but performing it before those algorithms produces normalized output which better preserves the intent of the original text."
    pub struct CJKNormalization;
    impl Normalizer for CJKNormalization {
        fn normalize(&self, s: &str) -> String {
            s.cjk_compat_variants().collect()
        }
    }

    /// Decomposes a string, then removes non-ascii code points.
    ///
    /// ```
    /// # use fuzzywuzzy::normalization::{Normalizer, UnicodeToAsciiNormalization};
    /// // U+00E4
    /// assert_eq!(UnicodeToAsciiNormalization.normalize("ä"), "a");
    /// // U+0061 + U+0308
    /// assert_eq!(UnicodeToAsciiNormalization.normalize("a\u{0308}"), "a");
    /// ```
    pub struct UnicodeToAsciiNormalization;
    impl Normalizer for UnicodeToAsciiNormalization {
        fn normalize(&self, s: &str) -> String {
            s.nfd().filter(char::is_ascii).collect()
        }
    }
}
