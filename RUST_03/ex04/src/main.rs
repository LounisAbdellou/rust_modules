use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
struct Time {
    hours: u32,
    minutes: u32,
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hours: {}, minutes: {};", self.hours, self.minutes)
    }
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingColon => write!(f, "':' missing."),
            Self::InvalidLength => write!(f, "invalid length."),
            Self::InvalidNumber => write!(f, "invalid number."),
        }
    }
}

impl FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hours = 0;
        let mut minutes = 0;
        let mut is_hours = true;
        let bytes = s.as_bytes();

        if bytes.len() != 5 {
            return Err(TimeParseError::InvalidLength);
        } else if bytes[2] != b':' {
            return Err(TimeParseError::MissingColon);
        }

        for byte in bytes {
            if *byte == b':' && is_hours {
                is_hours = false;
                continue;
            } else if byte.is_ascii_digit() && is_hours {
                hours = (hours * 10) + (*byte as u32 - 48);
            } else if byte.is_ascii_digit() && !is_hours {
                minutes = (minutes * 10) + (*byte as u32 - 48);
            } else {
                return Err(TimeParseError::InvalidNumber);
            }
        }

        if hours >= 24 || minutes >= 60 {
            return Err(TimeParseError::InvalidNumber);
        }

        Ok(Self { hours, minutes })
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
