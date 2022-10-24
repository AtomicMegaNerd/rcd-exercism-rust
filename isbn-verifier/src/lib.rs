/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }
    let mut digits = isbn
        .chars()
        .filter_map(|ch| match ch {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => ch.to_digit(10),
            _ => None,
        })
        .collect::<Vec<u32>>();
    if isbn.ends_with('X') {
        digits.push(10);
    }
    if digits.len() != 10 {
        return false;
    }
    do_isbn_checksum(&digits)
}

fn do_isbn_checksum(digits: &[u32]) -> bool {
    let mut check_sum_mul = 10;
    let checksum = digits.iter().fold(0, |acc, d| {
        let res = acc + (d * check_sum_mul);
        check_sum_mul -= 1;
        res
    });
    checksum % 11 == 0
}
