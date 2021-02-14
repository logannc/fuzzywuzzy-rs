#![deny(warnings)]
//! Fuzzy string matching like
//! [FuzzyWuzzy.py](https://github.com/seatgeek/fuzzywuzzy) like a boss. It uses
//! [Levenshtein Distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
//! to calculate the differences between sequences in a simple-to-use package.

#[macro_use]
pub mod utils;
pub mod fuzz;
pub mod normalization;
pub mod primitives;
pub mod process;
pub mod segmentation;
