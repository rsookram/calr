extern crate chrono;
extern crate structopt;

mod cal;

use cal::Month;
use chrono::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "calendarust")]
struct Opt {
    /// Display the specified month.
    #[structopt(name = "month", short = "m")]
    month: Option<u32>,
}

fn main() {
    let opt = Opt::from_args();

    let now = Local::now().naive_local().date();

    let year = now.year();
    let month_number = opt.month.unwrap_or_else(|| now.month());
    if month_number < 1 || month_number > 12 {
        let error = format!(
            "calendarust: {} is not a month number (1..12)",
            month_number
        );
        exit_with_error_code(&error, 64)
    }

    let m = Month::new(year, month_number).expect("invalid time");
    println!("{}", m);
}

fn exit_with_error_code(err: &str, code: i32) {
    eprintln!("{}", err);
    std::process::exit(code);
}
