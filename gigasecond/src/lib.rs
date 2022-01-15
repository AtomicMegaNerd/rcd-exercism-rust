use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::new(1_000_000_000, 0);
    start.checked_add(gigasecond).unwrap()
}
