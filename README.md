# fuzzyrusty
Fuzzy string matching like a boss. It uses Levenshtein Distance to calculate the differences between sequences in a simple-to-use package.

## Installation 
`fuzzyrusty` is currently only available through GitHub. Add this to your `Cargo.toml`:
```toml
[dependencies]
fuzzyrusty = { git = "https://github.com/logannc/fuzzyrusty", branch = "master" }
```

## Documentation
Clone the repository and run `$ cargo doc --open`.

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