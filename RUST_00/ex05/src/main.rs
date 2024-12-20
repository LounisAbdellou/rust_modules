fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!("Invalid year given");
    } else if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        return true;
    }

    return false;
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    if month > 12 {
        panic!("Invalid month given.");
    }

    match (month, month % 2, month <= 7, is_leap_year(year)) {
        (2, _, _, true) => return 29,
        (2, _, _, false) => return 28,
        (_, 0, false, _) => return 31,
        (_, 1, true, _) => return 31,
        (_, _, _, _) => return 30,
    }
}

fn get_month<'a>(month: u32) -> &'a str {
    match month {
        1 => return "January",
        2 => return "February",
        3 => return "March",
        4 => return "April",
        5 => return "May",
        6 => return "June",
        7 => return "July",
        8 => return "August",
        9 => return "September",
        10 => return "October",
        11 => return "November",
        12 => return "December",
        _ => return "Unknown",
    }
}

fn main() {
    let mut current_day = 1;

    for year in 1..=2024 {
        for month in 1..=12 {
            for days in 1..=num_days_in_month(year, month) {
                if current_day % 5 == 0 && days == 13 {
                    println!("Friday, {} {}, {}", get_month(month), days, year);
                }

                current_day = if current_day < 7 { current_day + 1 } else { 1 };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_years() {
        assert_eq!(is_leap_year(1600), true);
        assert_eq!(is_leap_year(2004), true);
    }

    #[test]
    fn common_years() {
        assert_eq!(is_leap_year(1500), false);
        assert_eq!(is_leap_year(2005), false);
    }

    #[test]
    fn february() {
        assert_eq!(num_days_in_month(1600, 2), 29);
        assert_eq!(num_days_in_month(2004, 2), 29);
        assert_eq!(num_days_in_month(1500, 2), 28);
        assert_eq!(num_days_in_month(2005, 2), 28);
    }

    #[test]
    fn other_months() {
        assert_eq!(num_days_in_month(1600, 1), 31);
        assert_eq!(num_days_in_month(2004, 3), 31);
        assert_eq!(num_days_in_month(1500, 11), 30);
        assert_eq!(num_days_in_month(2005, 7), 31);
        assert_eq!(num_days_in_month(2025, 6), 30);
    }

    #[test]
    #[should_panic]
    fn invalid_year() {
        is_leap_year(0);
    }

    #[test]
    #[should_panic]
    fn invalid_month() {
        num_days_in_month(2024, 300);
    }
}
