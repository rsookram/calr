extern crate chrono;

mod month;

use chrono::prelude::*;

fn main() {
    let now = Local::now().naive_local().date();
    month::print(now.year(), now.month());
}
