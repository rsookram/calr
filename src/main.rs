mod month;

use chrono::prelude::*;
use month::iter::MonthGenerator;
use month::Month;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "calr")]
struct Opt {
    /// Display the specified year.
    #[structopt(name = "year", short = "y")]
    year: Option<i32>,

    /// Display the specified month.
    #[structopt(name = "month", short = "m")]
    month: Option<u32>,

    /// Display the number of months after the current month.
    #[structopt(name = "months after", short = "A", default_value = "0")]
    months_after: u16,

    /// Display the number of months before the current month.
    #[structopt(name = "months before", short = "B", default_value = "0")]
    months_before: u16,
}

fn main() {
    let opt = Opt::from_args();

    let now = Local::now().naive_local().date();

    let output = months(now, opt)
        .map(|month| format!("{}", month))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);
}

fn months(now: NaiveDate, opt: Opt) -> impl Iterator<Item = Month> {
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
    let mut gen = MonthGenerator::new(m);

    if opt.months_before > 0 {
        gen.nth_prev(usize::from(opt.months_before) - 1);
    }

    gen.take(usize::from(opt.months_after + opt.months_before) + 1)
}

fn exit_with_error_code(err: &str, code: i32) {
    eprintln!("calr: {}", err);
    std::process::exit(code);
}
