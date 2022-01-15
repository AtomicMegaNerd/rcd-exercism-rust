use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut matches = HashSet::<&str>::new();
    let word_lower = word.to_lowercase();
    let word_cmp = return_sorted_string(&word_lower);
    for candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();
        let candidate_cmp = return_sorted_string(&candidate_lower);
        if (candidate_cmp == word_cmp) && (candidate_lower != word_lower) {
            matches.insert(candidate);
        }
    }
    matches
}

fn return_sorted_string(word: &str) -> String {
    let mut word_vec = word.chars().collect::<Vec<char>>();
    word_vec.sort_unstable();
    word_vec.into_iter().collect::<String>()
}
