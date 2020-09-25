# fuzzywuzzy-rs

[![docs.rs badge](https://docs.rs/fuzzywuzzy/badge.svg)](https://docs.rs/fuzzywuzzy) [![crates.io badge](https://img.shields.io/crates/v/fuzzywuzzy.svg)](https://crates.io/crates/fuzzywuzzy)

> Fuzzy string matching like a boss. It uses Levenshtein Distance to calculate the differences between sequences in a simple-to-use package.
> [fuzzywuzzy](https://github.com/seatgeek/fuzzywuzzy)

This is a Rust port of the Python package fuzzywuzzy. We aim to be drop-in replacement for the original.

At the time of writing, our matching algorithm is based on the difflib implementation results which may, in rare cases, [have slightly different results](https://github.com/seatgeek/fuzzywuzzy/issues/128) compared to the python Levenshtein implementation.

**Note: This project was originally named `fuzzyrusty`. Someone else cloned and published it to crates.io https://crates.io/crates/fuzzyrusty. _We do not control that crate._ This is why we have changed the name.**

## Installation
`fuzzywuzzy` is currently available through GitHub or crates.io.

For the latest stable release, add this to your `Cargo.toml`:

```toml
[dependencies]
fuzzywuzzy = "*"
```

For the bleeding edge, you can pull directly from master:

```toml
[dependencies]
fuzzywuzzy = { git = "https://github.com/logannc/fuzzywuzzy-rs", branch = "master" }
```

## Documentation
Clone the repository and run `$ cargo doc --open`, or visit [docs.rs](https://docs.rs/crate/fuzzywuzzy/0.0.2).

## Usage
### Simple Ratio
```rust
assert_eq!(fuzz::ratio("this is a test", "this is a test!"), 97);
```
### Partial Ratio
```rust
assert_eq!(fuzz::partial_ratio("this is a test", "this is a test!"), 100);
```
### Token Sort Ratio
```rust
assert_eq!(fuzz::ratio("fuzzy wuzzy was a bear", "wuzzy fuzzy was a bear"), 91);
assert_eq!(fuzz::token_sort_ratio("fuzzy wuzzy was a bear", "wuzzy fuzzy was a bear", true, true), 100);
```
### Token Set Ratio
```rust
assert_eq!(fuzz::ratio("fuzzy was a bear", "fuzzy fuzzy was a bear"), 84);
assert_eq!(fuzz::token_set_ratio("fuzzy was a bear", "fuzzy fuzzy was a bear", true, true), 100);
```
### Process
```rust
assert_eq!(process::extract_one(
  "cowboys",
 &["Atlanta Falcons", "Dallas Cowboys", "New York Jets"],
 &utils::full_process,
 &fuzz::wratio,
  0,
), Some(("Dallas Cowboys".to_string(), 90)));
```
