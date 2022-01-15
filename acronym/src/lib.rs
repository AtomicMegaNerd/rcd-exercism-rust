// This is grotesque, but it works
pub fn abbreviate(phrase: &str) -> String {
    add_space_to_camel_case_words(phrase)
        .split(|c: char| c.is_whitespace() || c == '-')
        .map(|word| {
            word.chars()
                // This should be the first letter in each split string...
                .find(|ch| ch.is_ascii_alphabetic())
                // Instead of fiddling with optional let's just replace
                // None with a default character we can rip out later with filter...
                .unwrap_or_default()
        })
        // This filter rips out any default characters that were put there by
        // unwrap_or_default()
        .filter(|ch| ch.is_ascii_alphabetic())
        .collect::<String>()
        .to_uppercase()
}

// This is also grotesque, but it also works
fn add_space_to_camel_case_words(phrase: &str) -> String {
    let mut prev_char = '\0';
    let mut new_str = String::new();
    // there is no windows() method on &str so this is another way to do it.
    phrase.chars().for_each(|ch: char| {
        // If we have a lowercase followed by an uppercase this is a CamelCase word.
        if prev_char.is_ascii_lowercase() && ch.is_ascii_uppercase() {
            // Split the CamelCase word with a space
            new_str += &format!("{} {}", prev_char, ch);
        } else {
            new_str += &format!("{}{}", prev_char, ch);
        }
        prev_char = ch;
    });
    new_str
}
