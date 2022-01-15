use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // First, if the nucleotide argument is not valid error out
    is_nucleotide(nucleotide)?;

    // Next, error out if any of the characters in the DNA string are not
    // valid nucleotide.
    let nucleotides: String = dna
        .to_uppercase()
        .chars()
        .map(is_nucleotide)
        .collect::<Result<String, char>>()?; // collect() will put the String inside the Result!

    // Finally, filter out the other valid nucleotides so that we can
    // count the number of times the nucleotide we are interested in occurs
    // in the DNA string
    Ok(nucleotides
        .chars()
        .filter(|ch| ch == &nucleotide)
        .collect::<String>()
        .len())
}

/// This function returns the character wrapped in OK if it is a
/// nucleotide.  Otherwise it returns the character wrapped in an Err.
fn is_nucleotide(ch: char) -> Result<char, char> {
    match ch {
        'A' | 'C' | 'T' | 'G' => Ok(ch),
        _ => Err(ch),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let res_map = HashMap::from([
        ('A', count('A', dna)?),
        ('C', count('C', dna)?),
        ('G', count('G', dna)?),
        ('T', count('T', dna)?),
    ]);
    Ok(res_map)
}
