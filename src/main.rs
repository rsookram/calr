extern crate chrono;

use chrono::prelude::*;
use std::ops::RangeInclusive;

const DAY_OF_WEEK_HEADER: &str = "Su Mo Tu We Th Fr Sa";

fn main() {
    let now = Local::now();

    println!("{:^20}", month_header(&now));
    println!("{}", DAY_OF_WEEK_HEADER);

    // Layout hardcoded for March 2019
    println!("{:>20}", week(1..=2));
    println!("{}", week(3..=9));
    println!("{}", week(10..=16));
    println!("{}", week(17..=23));
    println!("{}", week(24..=30));
    println!("{:<20}", week(31..=31));
}

fn month_header(d: &DateTime<Local>) -> String {
    format!("{} {}", month(d), d.year())
}

fn month(d: &DateTime<Local>) -> &str {
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

fn week(days: RangeInclusive<u8>) -> String {
    days.map(|d| format!("{:2}", d))
        .collect::<Vec<String>>()
        .join(" ")
}

fn weekday_for_first(d: &DateTime<Local>) -> Weekday {
    d.with_day0(0).unwrap().weekday()
}

fn num_days_in_month(d: &DateTime<Local>) -> u8 {
    for i in (28u8..=31u8).rev() {
        if let Some(_) = d.with_day(u32::from(i)) {
            return i;
        }
    }

    panic!("unknown number of days in month {}", d.month());
}
