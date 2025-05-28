pub mod iter;

use std::cmp::min;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Write;
use std::ops::RangeInclusive;
use time::Date;
use time::Weekday;

// Trailing spaces are for consistency with cal
const TRAILING_SPACE: &str = "  ";
const DAY_OF_WEEK_HEADER: &str = "Su Mo Tu We Th Fr Sa";

/// A calendar month associated with a specific year
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
pub struct Month {
    date: Date,
}

impl Month {
    /// Creates a new `Month` associated with the given `year` and `month`.
    ///
    /// Returns `None` on an out-of-range year or invalid month.
    ///
    /// ```
    /// use calr::month::Month;
    ///
    /// assert!(Month::new(2020, 0).is_none());
    /// assert!(Month::new(2020, 1).is_some());
    ///
    /// assert!(Month::new(2020, 12).is_some());
    /// assert!(Month::new(2020, 13).is_none());
    /// ```
    pub fn new(year: i32, month: u8) -> Option<Month> {
        let date = Date::from_calendar_date(year, month.try_into().ok()?, 1).ok()?;
        Some(Month { date })
    }

    /// The year this `Month` is associated with.
    ///
    /// ```
    /// use calr::month::Month;
    ///
    /// assert_eq!(Month::new(2020, 1).unwrap().year(), 2020);
    /// ```
    pub fn year(&self) -> i32 {
        self.date.year()
    }

    /// Returns the month number starting from 1.
    ///
    /// The return value ranges from 1 to 12.
    ///
    /// ```
    /// use calr::month::Month;
    ///
    /// assert_eq!(Month::new(2020, 1).unwrap().month_number(), 1);
    /// assert_eq!(Month::new(2020, 12).unwrap().month_number(), 12);
    /// ```
    pub fn month_number(&self) -> u8 {
        self.date.month() as u8
    }

    /// Allocates a `String` to display as the header of this `Month`
    fn month_header(&self) -> String {
        format!("{} {}", self.date.month(), self.date.year())
    }

    /// Allocates a `String` to display the weeks of this `Month`
    fn weeks(&self) -> String {
        let mut result = String::new();

        let layout = self.layout_weeks();
        let last_week = layout.len() - 1;
        for (i, w) in layout.into_iter().enumerate() {
            if i == 0 {
                write!(result, "{:>20}", Month::week(w)).expect("write to String");
            } else if i == last_week {
                write!(result, "{:<20}", Month::week(w)).expect("write to String");
            } else {
                result.push_str(&Month::week(w));
            }
            result.push_str(TRAILING_SPACE);
            result.push('\n');
        }

        result
    }

    fn layout_weeks(&self) -> Vec<RangeInclusive<u8>> {
        let initial_weekday = self.weekday_for_first();

        let last_day_in_month = self.date.month().length(self.date.year());
        let mut days_remaining = last_day_in_month;
        let mut start = 1;

        let mut result = vec![];

        let days_in_first_week = 7 - initial_weekday.number_days_from_sunday();
        result.push(start..=days_in_first_week);
        start += days_in_first_week;
        days_remaining -= days_in_first_week;

        while days_remaining > 0 {
            let end_of_week = min(start + 6, last_day_in_month);
            result.push(start..=end_of_week);
            start += 7;
            days_remaining = if days_remaining >= 7 {
                days_remaining - 7
            } else {
                0
            };
        }

        result
    }

    fn week(days: RangeInclusive<u8>) -> String {
        days.map(|d| format!("{:2}", d))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn weekday_for_first(&self) -> Weekday {
        self.date
            .replace_day(1)
            .expect("every month has a first")
            .weekday()
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:^20}", self.month_header())?;
        writeln!(f, "{TRAILING_SPACE}")?;
        write!(f, "{DAY_OF_WEEK_HEADER}")?;
        writeln!(f, "{TRAILING_SPACE}")?;
        write!(f, "{}", self.weeks())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_year() {
        let m = Month::new(262144, 1);
        assert_eq!(None, m);
    }

    #[test]
    fn invalid_month() {
        let m = Month::new(2019, 15);
        assert_eq!(None, m);
    }

    #[test]
    fn january() {
        let m = Month::new(2019, 1).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |    January 2019      |
            |Su Mo Tu We Th Fr Sa  |
            |       1  2  3  4  5  |
            | 6  7  8  9 10 11 12  |
            |13 14 15 16 17 18 19  |
            |20 21 22 23 24 25 26  |
            |27 28 29 30 31        |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn february_leap_year() {
        let m = Month::new(2016, 2).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |   February 2016      |
            |Su Mo Tu We Th Fr Sa  |
            |    1  2  3  4  5  6  |
            | 7  8  9 10 11 12 13  |
            |14 15 16 17 18 19 20  |
            |21 22 23 24 25 26 27  |
            |28 29                 |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn february_not_leap_year() {
        let m = Month::new(2019, 2).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |   February 2019      |
            |Su Mo Tu We Th Fr Sa  |
            |                1  2  |
            | 3  4  5  6  7  8  9  |
            |10 11 12 13 14 15 16  |
            |17 18 19 20 21 22 23  |
            |24 25 26 27 28        |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn march() {
        let m = Month::new(2019, 3).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |     March 2019       |
            |Su Mo Tu We Th Fr Sa  |
            |                1  2  |
            | 3  4  5  6  7  8  9  |
            |10 11 12 13 14 15 16  |
            |17 18 19 20 21 22 23  |
            |24 25 26 27 28 29 30  |
            |31                    |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn april() {
        let m = Month::new(2019, 4).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |     April 2019       |
            |Su Mo Tu We Th Fr Sa  |
            |    1  2  3  4  5  6  |
            | 7  8  9 10 11 12 13  |
            |14 15 16 17 18 19 20  |
            |21 22 23 24 25 26 27  |
            |28 29 30              |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn may() {
        let m = Month::new(2019, 5).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |      May 2019        |
            |Su Mo Tu We Th Fr Sa  |
            |          1  2  3  4  |
            | 5  6  7  8  9 10 11  |
            |12 13 14 15 16 17 18  |
            |19 20 21 22 23 24 25  |
            |26 27 28 29 30 31     |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn june() {
        let m = Month::new(2019, 6).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |     June 2019        |
            |Su Mo Tu We Th Fr Sa  |
            |                   1  |
            | 2  3  4  5  6  7  8  |
            | 9 10 11 12 13 14 15  |
            |16 17 18 19 20 21 22  |
            |23 24 25 26 27 28 29  |
            |30                    |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn july() {
        let m = Month::new(2019, 7).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |     July 2019        |
            |Su Mo Tu We Th Fr Sa  |
            |    1  2  3  4  5  6  |
            | 7  8  9 10 11 12 13  |
            |14 15 16 17 18 19 20  |
            |21 22 23 24 25 26 27  |
            |28 29 30 31           |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn august() {
        let m = Month::new(2019, 8).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |    August 2019       |
            |Su Mo Tu We Th Fr Sa  |
            |             1  2  3  |
            | 4  5  6  7  8  9 10  |
            |11 12 13 14 15 16 17  |
            |18 19 20 21 22 23 24  |
            |25 26 27 28 29 30 31  |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn september() {
        let m = Month::new(2019, 9).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |   September 2019     |
            |Su Mo Tu We Th Fr Sa  |
            | 1  2  3  4  5  6  7  |
            | 8  9 10 11 12 13 14  |
            |15 16 17 18 19 20 21  |
            |22 23 24 25 26 27 28  |
            |29 30                 |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn october() {
        let m = Month::new(2019, 10).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |    October 2019      |
            |Su Mo Tu We Th Fr Sa  |
            |       1  2  3  4  5  |
            | 6  7  8  9 10 11 12  |
            |13 14 15 16 17 18 19  |
            |20 21 22 23 24 25 26  |
            |27 28 29 30 31        |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn november() {
        let m = Month::new(2019, 11).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |   November 2019      |
            |Su Mo Tu We Th Fr Sa  |
            |                1  2  |
            | 3  4  5  6  7  8  9  |
            |10 11 12 13 14 15 16  |
            |17 18 19 20 21 22 23  |
            |24 25 26 27 28 29 30  |
            "#,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn december() {
        let m = Month::new(2019, 12).expect("invalid date");
        let actual = format!("{m}");
        let expected = from_formatted(
            r#"
            |   December 2019      |
            |Su Mo Tu We Th Fr Sa  |
            | 1  2  3  4  5  6  7  |
            | 8  9 10 11 12 13 14  |
            |15 16 17 18 19 20 21  |
            |22 23 24 25 26 27 28  |
            |29 30 31              |
            "#,
        );
        assert_eq!(expected, actual);
    }

    fn from_formatted(month_str: &str) -> String {
        month_str
            .split("\n")
            .filter(|line| line.contains("|"))
            .filter_map(|line| line.split("|").nth(1))
            .map(|line| line.to_string() + "\n")
            .collect()
    }
}
