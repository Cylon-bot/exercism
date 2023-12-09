use std::fmt;
#[derive(Debug)]
pub struct Clock {
    hours: u32,
    minutes: u32,
}
pub fn calculate_hour(hours: u32, prev_minutes: u32, minutes: i32) -> u32 {
    (hours as i32 + ((minutes + prev_minutes as i32) as f64 / 60.0).floor() as i32).rem_euclid(24)
        as u32
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: calculate_hour(hours.rem_euclid(24) as u32, 0, minutes),
            minutes: minutes.rem_euclid(60) as u32,
        }
    }
    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock {
            hours: calculate_hour(self.hours, self.minutes, minutes),
            minutes: (self.minutes as i32 + minutes).rem_euclid(60) as u32,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
