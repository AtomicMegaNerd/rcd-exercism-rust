/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    // Folds are fun!  This reminds me of Haskell :-)
    word.to_ascii_uppercase()
        .chars()
        .fold(0, |acc, ch| acc + score_for_letter(ch))
}

fn score_for_letter(letter: char) -> u64 {
    match letter {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    }
}
