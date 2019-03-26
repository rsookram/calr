extern crate chrono;

mod cal;

use cal::Month;
use chrono::prelude::*;

fn main() {
    let now = Local::now().naive_local().date();
    let m = Month::new(now.year(), now.month()).expect("invalid time");
    println!("{}", m);
}
