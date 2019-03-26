use chrono::prelude::*;
use std::cmp::min;
use std::fmt;
use std::ops::RangeInclusive;

const DAY_OF_WEEK_HEADER: &str = "Su Mo Tu We Th Fr Sa";

pub struct Month {
    date: NaiveDate,
}

impl Month {
    pub fn new(y: i32, m: u32) -> Option<Month> {
        let date = NaiveDate::from_ymd_opt(y, m, 1)?;
        Some(Month { date, })
    }

    fn month_header(&self) -> String {
        format!("{} {}", self.month_name(), self.date.year())
    }

    fn month_name(&self) -> &str {
        match self.date.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("invalid month {}", self.date.month()),
        }
    }

    fn weeks(&self) -> String {
        let mut result = String::new();

        let layout = self.layout_weeks();
        let last_week = layout.len() - 1;
        for (i, w) in layout.into_iter().enumerate() {
            // Trailing spaces are for consistency with cal
            if i == 0 {
                result.push_str(&format!("{:>20}  ", Month::week(w)));
            } else if i == last_week {
                result.push_str(&format!("{:<20}  ", Month::week(w)));
            } else {
                result.push_str(&format!("{}  ", Month::week(w)));
            }
            result.push('\n');
        }

        result
    }

    fn layout_weeks(&self) -> Vec<RangeInclusive<u32>> {
        let initial_weekday = self.weekday_for_first();

        let last_day_in_month = self.num_days_in_month();
        let mut days_remaining = last_day_in_month;
        let mut start = 1;

        let mut result = vec![];

        let days_in_first_week = 7 - initial_weekday.num_days_from_sunday();
        result.push(start..=days_in_first_week);
        start += days_in_first_week;
        days_remaining -= days_in_first_week;

        while days_remaining > 0 {
            let end_of_week = min(start + 6, last_day_in_month);
            result.push(start..=end_of_week);
            start += 7;
            days_remaining = if days_remaining >= 7 { days_remaining - 7 } else { 0 };
        }

        result
    }

    fn week(days: RangeInclusive<u32>) -> String {
        days.map(|d| format!("{:2}", d))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn weekday_for_first(&self) -> Weekday {
        self.date.with_day0(0).unwrap().weekday()
    }

    fn num_days_in_month(&self) -> u32 {
        (28..=31)
            .rev()
            .filter(|&i| self.date.with_day(i).is_some())
            .nth(0)
            .unwrap_or_else(|| panic!("unknown number of days in month {}", self.date.month()))
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // Trailing spaces are for consistency with cal
        writeln!(f, "{:^20}  ", self.month_header())?;
        writeln!(f, "{}  ", DAY_OF_WEEK_HEADER)?;
        write!(f, "{}", self.weeks())
    }
}
