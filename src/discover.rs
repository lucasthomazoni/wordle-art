use std::collections::HashMap;
use crate::patterns::Pattern;
use crate::wordle::score;

/// Build an index mapping each achievable pattern to the words that produce it.
pub fn build_index(answer: &str, words: &[String]) -> HashMap<Pattern, Vec<String>> {
    let mut index: HashMap<Pattern, Vec<String>> = HashMap::new();
    for w in words {
        let p = score(answer, w);
        index.entry(p).or_default().push(w.clone());
    }
    index
}

/// Return all achievable (pattern, example_word) pairs, sorted by pattern.
pub fn discover_patterns(index: &HashMap<Pattern, Vec<String>>) -> Vec<(Pattern, String)> {
    let mut results: Vec<(Pattern, String)> = index
        .iter()
        .map(|(p, words)| (*p, words[0].clone()))
        .collect();
    results.sort_by_key(|(p, _)| *p);
    results
}
