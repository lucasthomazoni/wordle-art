use rand::prelude::IndexedRandom;

use crate::wordle::score;
use crate::patterns::{Pattern, format_pattern};

/// Find all words from the list that produce the given pattern against the answer.
pub fn find_words(answer: &str, pattern: &Pattern, words: &[String]) -> Vec<String> {
    words
        .iter()
        .filter(|w| score(answer, w) == *pattern)
        .cloned()
        .collect()
}

/// For each target pattern, pick a random matching word. Returns the list of guesses + the answer.
/// GGGGG rows are skipped since the final row (the answer itself) is always all-green.
pub fn build_art(answer: &str, patterns: &[Pattern], words: &[String]) -> Result<Vec<String>, String> {
    let all_green: Pattern = [2, 2, 2, 2, 2];
    let mut rng = rand::rng();
    let mut result = Vec::new();
    for (i, p) in patterns.iter().enumerate() {
        if *p == all_green {
            continue;
        }
        let matches: Vec<_> = find_words(answer, p, words)
            .into_iter()
            .filter(|w| w != answer)
            .collect();
        if matches.is_empty() {
            return Err(format!(
                "No word found for row {} with pattern {} — try a different answer or pattern",
                i + 1,
                format_pattern(p)
            ));
        }
        result.push(matches.choose(&mut rng).unwrap().clone());
    }
    // The last row is always the answer itself (all green)
    result.push(answer.to_string());
    Ok(result)
}
