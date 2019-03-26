extern crate chrono;

mod month;

use chrono::prelude::*;

fn main() {
    let now = Local::now().naive_local().date();
    let m = month::Month::new(now.year(), now.month()).expect("invalid time");
    println!("{}", m);
}
