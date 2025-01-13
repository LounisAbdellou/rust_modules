struct Time {
    hours: u32,
    minutes: u32,
}

enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

trait Parser {
    fn parse<T>() -> Result<Time, TimeParseError>;
}

impl Parser for Time {
    fn parse<T>() -> Result<Time, TimeParseError> {}
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
