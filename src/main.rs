extern crate chrono;

use chrono::prelude::*;
use std::cmp::min;
use std::ops::RangeInclusive;

const DAY_OF_WEEK_HEADER: &str = "Su Mo Tu We Th Fr Sa";

fn main() {
    let now = Local::now().naive_local().date();
    print(now.year(), now.month());
}

fn print(y: i32, m: u32) {
    let d = NaiveDate::from_ymd(y, m, 1);
    // Trailing spaces are for consistency with cal
    println!("{:^20}  ", month_header(&d));
    println!("{}  ", DAY_OF_WEEK_HEADER);
    print!("{}", weeks(&d));
}

fn month_header(d: &NaiveDate) -> String {
    format!("{} {}", month(d), d.year())
}

fn month(d: &NaiveDate) -> &str {
    match d.month() {
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
        _ => panic!("invalid month {}", d.month()),
    }
}

fn weeks(d: &NaiveDate) -> String {
    let mut result = String::new();

    let layout = layout_weeks(d);
    let last_week = layout.len() - 1;
    for (i, w) in layout.into_iter().enumerate() {
        // Trailing spaces are for consistency with cal
        if i == 0 {
            result.push_str(&format!("{:>20}  ", week(w)));
        } else if i == last_week {
            result.push_str(&format!("{:<20}  ", week(w)));
        } else {
            result.push_str(&format!("{}  ", week(w)));
        }
        result.push('\n');
    }

    result
}

fn layout_weeks(d: &NaiveDate) -> Vec<RangeInclusive<u32>> {
    let initial_weekday = weekday_for_first(d);

    let last_day_in_month = num_days_in_month(d);
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

fn weekday_for_first(d: &NaiveDate) -> Weekday {
    d.with_day0(0).unwrap().weekday()
}

fn num_days_in_month(d: &NaiveDate) -> u32 {
    for i in (28..=31).rev() {
        if let Some(_) = d.with_day(i) {
            return i;
        }
    }

    panic!("unknown number of days in month {}", d.month());
}
