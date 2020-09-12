/// Select the best match in a list or dictionary of choices.
///
/// Find best matches in a list or dictionary of choices, return a generator of tuples containing
/// the match and its score. If a dictionary is used, also returns the key for each match.
///
/// TODO: Add support for choices as HashMap<&str, &str>, not only as slice &[&str].
pub fn extract_without_order(
    query: &str,
    choices: &[&str],
    processor: &dyn Fn(&str, bool) -> String,
    scorer: &dyn Fn(&str, &str, bool, bool) -> u8,
    score_cutoff: u8
) -> Vec<(String, u8)> {
    if choices.is_empty() {
        return vec![];
    }

    let processed_query: String = processor(query, false);
    if processed_query.is_empty() {
        println!("Applied processor reduces input query to empty string, all comparisons will have score 0. [Query: '{0}']", processed_query.as_str());
    }

    // TODO: Check if scorer in list of known processor functions to avoid calling utils::full_process multiple times.
    // TODO: Only process the query once instead of for every choice.

    let mut results = vec![];
    for choice in choices {
        let processed: String = processor(choice, false);
        let score: u8 = scorer(processed_query.as_str(), processed.as_str(), true, true);
        if score >= score_cutoff {
            results.push((choice.to_string(), score))
        }
    }
    results
}

/// Find the single best match above a score in a list of choices.
///
/// This is a convenience method which returns the single best choice.
///
/// TODO: Add support for choices as HashMap<&str, &str>, not only as slice &[&str].
pub fn extract_one(
    query: &str,
    choices: &[&str],
    processor: &dyn Fn(&str, bool) -> String,
    scorer: &dyn Fn(&str, &str, bool, bool) -> u8,
    score_cutoff: u8
) -> Option<(String, u8)> {
    let best = extract_without_order(query, choices, processor, scorer, score_cutoff);
    if best.is_empty() {
        return None
    }
    best.iter().cloned().max_by(|(_, acc_score), (_, score)| {
        acc_score.cmp(score)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils;
    use fuzz;

    mod process {
        use super::*;

        fn get_baseball_strings() -> &'static [&'static str] {
            &[
                "new york mets vs chicago cubs",
                "chicago cubs vs chicago white sox",
                "philladelphia phillies vs atlanta braves",
                "braves vs mets",
            ]
        }

        // Call extract_one, unwrap the option, and return 0th element (the choice).
        fn unwrap_extract_one_choice(query: &str) -> String {
            // Specify sane defaults.
            extract_one(query, get_baseball_strings(), &utils::full_process, &fuzz::wratio, 0).unwrap().0
        }

        #[test]
        fn test_get_best_choice1() {
            let query = "new york mets at atlanta braves";
            let best = unwrap_extract_one_choice(query);
            assert_eq!(best.as_str(), get_baseball_strings()[3])
        }

        #[test]
        fn test_get_best_choice2() {
            let query = "philadelphia phillies at atlanta braves";
            let best = unwrap_extract_one_choice(query);
            assert_eq!(best.as_str(), get_baseball_strings()[2])
        }

        #[test]
        fn test_get_best_choice3() {
            let query = "atlanta braves at philadelphia phillies";
            let best = unwrap_extract_one_choice(query);
            assert_eq!(best.as_str(), get_baseball_strings()[2])
        }

        #[test]
        fn test_get_best_choice4() {
            let query = "chicago cubs vs new york mets";
            let best = unwrap_extract_one_choice(query);
            assert_eq!(best.as_str(), get_baseball_strings()[0])
        }
    }
}