pub mod utils;
pub mod fuzz;
pub mod process;

#[cfg(test)]
mod tests {
    use fuzz;
    use utils;
    #[test]
    fn identity() {
        assert_eq!(fuzz::ratio("hello", "hello"), 100);
    }

    #[test]
    fn world() {
        assert_eq!(fuzz::ratio("hello test", "hello world"), 57);
        assert_eq!(fuzz::ratio("hello test", "hello worlasdfasd"), 52);
    }

    #[test]
    fn case_insensitive() {
        assert!(fuzz::ratio("hello WORLD", "hello world") != 100);
        assert_eq!(fuzz::ratio(&utils::full_process("hello WORLD", false),
                               &utils::full_process("hello world", false)),
                   100);
    }

    #[test]
    fn token_sort() {
        assert_eq!(fuzz::token_sort_ratio("hello world", "world hello", false, false),
                   100);
    }

    #[test]
    fn partial() {
        assert_eq!(fuzz::partial_ratio("hello", "hello world"), 100);
    }

    #[test]
    fn partial_token_sort() {
        assert_eq!(fuzz::partial_token_set_ratio("new york mets vs atlanta braves",
                                                 "atlanta braves vs new york mets",
                                                 false,
                                                 false),
                   100);
    }

    #[test]
    fn token_set() {
        assert_eq!(fuzz::token_set_ratio("new york mets vs atlanta braves",
                                         "atlanta braves vs new york mets",
                                         false,
                                         false),
                   100);
    }

    #[test]
    fn partial_token_set() {
        assert_eq!(fuzz::partial_token_set_ratio("new york mets vs atlanta braves",
                                                 "new york city mets - atlanta braves",
                                                 false,
                                                 false),
                   100);
    }
}
