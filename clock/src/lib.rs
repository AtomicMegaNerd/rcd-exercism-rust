use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = minutes;
        let mut hours = hours;

        let minutes_mod = minutes.rem_euclid(60);
        // There is some funky logic when calculating extra hours for
        // negative numbers
        let mut extra_hours = minutes / 60;
        if minutes < 0 && minutes_mod != 0 {
            extra_hours -= 1
        };
        minutes = minutes_mod;

        hours += extra_hours;
        hours = hours.rem_euclid(24);

        Clock {
            hours: hours.abs(),
            minutes: minutes.abs(),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
