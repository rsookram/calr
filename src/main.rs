mod cal;

use cal::Month;
use chrono::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "calendarust")]
struct Opt {
    /// Display the specified year.
    #[structopt(name = "year", short = "y")]
    year: Option<i32>,

    /// Display the specified month.
    #[structopt(name = "month", short = "m")]
    month: Option<u32>,
}

fn main() {
    let opt = Opt::from_args();

    let now = Local::now().naive_local().date();

    let year = opt.year.unwrap_or_else(|| now.year());
    if year < 1 || year > 9999 {
        let error = format!("year `{}' is not in range 1..9999", year);
        exit_with_error_code(&error, 64)
    }

    let month_number = opt.month.unwrap_or_else(|| now.month());
    if month_number < 1 || month_number > 12 {
        let error = format!("{} is not a month number (1..12)", month_number);
        exit_with_error_code(&error, 64)
    }

    let m = Month::new(year, month_number).expect("invalid time");
    println!("{}", m);
}

fn exit_with_error_code(err: &str, code: i32) {
    eprintln!("calendarust: {}", err);
    std::process::exit(code);
}
