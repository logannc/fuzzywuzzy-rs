//! Convenience methods to process fuzzy matching queries for common use cases.

use crate::fuzzywuzzy_compatible::utils::full_process;

pub fn default_processor(s: &str) -> String {
    full_process(s, false)
}

/// TODO: document and Move me to primitives
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Score {
    score: u8,
}

impl Score {
    const MAX: u8 = 100;
    pub fn new(score: u8) -> Option<Self> {
        if score <= Self::MAX {
            Some(Score { score })
        } else {
            None
        }
    }
    pub fn get_score(&self) -> u8 {
        self.score
    }
}

// TODO: where should i live? types? primitives?
#[derive(PartialEq, Eq, Debug)]
pub struct Match<T> {
    pub item: T,
    pub score: Score,
}

pub trait Processor<A, B> {
    fn process(&self, item: A) -> B;
}

impl<A, B, F: Fn(A) -> B> Processor<A, B> for F {
    fn process(&self, item: A) -> B {
        self(item)
    }
}
pub trait Scorer<A, B> {
    fn score(&self, query: &A, choice: B) -> Score;
}

impl<A, B, F: Fn(&A, B) -> Score> Scorer<A, B> for F {
    fn score(&self, query: &A, choice: B) -> Score {
        self(query, choice)
    }
}

/// ```
/// # use fuzzywuzzy::fuzzywuzzy_compatible::process::extract_without_order;
/// let query = "bar";
/// let choices = vec!["foo", "bar", "baz"];
/// assert_eq!(extract_without_order(query, &choices), vec![]);
/// ```
pub fn extract_without_order<'a>(
    query: &'a str,
    choices: &'a [&'a str],
) -> Vec<Match<&'a &'a str>> {
    extract_without_order_full(
        query,
        choices,
        |c: &'a &'a str| default_processor(*c),
        |q: &&str, c| {
            if *q == c {
                Score::new(100).unwrap()
            } else {
                Score::new(0).unwrap()
            }
        },
        Score::new(0).unwrap(),
    )
}

// pub fn checking() {
//     let query = "test";
//     let foo = vec!["a", "b", "c"];
//     extract_without_order_full(
//         &query,
//         &foo,
//         |&_a| default_processor(_a),
//         |_a, _b| Score::new(0),
//         Score::new(0),
//     );
// }

pub fn extract_without_order_full<'a, 'b, A: 'a, B, C>(
    query: A,
    choices: &'b [B],
    processor: impl Processor<&'b B, C>,
    scorer: impl Scorer<A, C>,
    score_cutoff: Score,
) -> Vec<Match<&'b B>>
// P: Processor<&'a B, C>,
    // S: Scorer<&'a A, C>, // how can this reference lifetime of function??? i.e., query: A instead of &'a A
    // but Scorer needs <&A> because I don't have a clone bound but the lifetime needs to be the lifetime of the function
{
    if choices.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (c, cs) in choices.iter().map(|c| (c, processor.process(c))) {
        let score = scorer.score(&query, cs);
        if score >= score_cutoff {
            result.push(Match { item: c, score });
        }
    }
    result
}

// pub fn extract_without_order_full<'a, A: 'a, B: 'a, C, P, S>(
//     query: &'a A,
//     choices: &'a [B],
//     processor: P,
//     scorer: S,
//     score_cutoff: Score,
// ) -> Vec<Match<&'a B>>
// where
//     A: ?Sized,
//     P: Processor<&'a B, C>,
//     S: Scorer<&'a A, C>, // how can this reference lifetime of function??? i.e., query: A instead of &'a A
//     // but Scorer needs <&A> because I don't have a clone bound but the lifetime needs to be the lifetime of the function
// {
//     if choices.is_empty() {
//         return Vec::new();
//     }
//     choices
//         .iter()
//         .map(|c| (c, processor.process(c)))
//         .map(|(c, cs)| {
//             // let qref: &'a A = &query;
//             Match {
//                 item: c,
//                 score: scorer.score(query, cs),
//             }
//         })
//         .filter(|m| m.score >= score_cutoff)
//         .collect()
// }

// pub fn extract_without_order_full<'b, 'a, A: 'b, B: 'a, C, P, S>(
//     query: A,
//     choices: &'a [B],
//     processor: P,
//     scorer: S,
//     score_cutoff: Score,
// ) -> Vec<Match<&'a B>>
// where
//     A: ?Sized,
//     P: Processor<&'a B, C>,
//     S: Scorer<&'b A, C>, // how can this reference lifetime of function??? i.e., query: A instead of &'a A
//                          // but Scorer needs <&A> because I don't have a clone bound but the lifetime needs to be the lifetime of the function
// {
//     if choices.is_empty() {
//         return Vec::new();
//     }
//     let result = {
//         let qref = &query;
//         choices
//             .iter()
//             .map(|c| (c, processor.process(c)))
//             .map(|(c, cs)| {
//                 // let qref: &'a A = &query;
//                 Match {
//                     item: c,
//                     score: scorer.score(qref, cs),
//                 }
//             })
//             .filter(|m| m.score >= score_cutoff)
//             .collect()
//     };
//     result
// }

// pub fn extract_without_order_full<'a, A: 'a, B: 'a, C, P, S>(
//     query: &'a A,
//     choices: &'a [B],
//     processor: P,
//     scorer: S,
//     score_cutoff: Score,
// ) -> Vec<Match<&'a B>>
// where
//     P: Processor<&'a B, C>,
//     S: Scorer<&'a A, C>,
// {
//     if choices.is_empty() {
//         return Vec::new();
//     }
//     choices
//         .iter()
//         .map(|c| (c, processor.process(c)))
//         .map(|(c, cs)| {
//             // let qref: &'a A = &query;
//             Match {
//                 item: c,
//                 score: scorer.score(query, cs),
//             }
//         })
//         .filter(|m| m.score >= score_cutoff)
//         .collect()
// }
