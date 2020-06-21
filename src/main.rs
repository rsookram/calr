mod error;

use calr::month::iter::MonthGenerator;
use calr::month::Month;
use chrono::prelude::*;
use error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "calr", author, about)]
struct Opt {
    /// Display the specified year [default: current]
    #[structopt(name = "year", short = "y")]
    year: Option<i32>,

    /// Display the specified month [default: current]
    #[structopt(name = "month", short = "m")]
    month: Option<u32>,

    /// Display the number of months after the current month
    #[structopt(name = "months after", short = "A", default_value = "0")]
    months_after: u16,

    /// Display the number of months before the current month
    #[structopt(name = "months before", short = "B", default_value = "0")]
    months_before: u16,
}

fn main() {
    let opt = Opt::from_args();

    let now = Local::now().naive_local().date();

    let months = months(now, &opt).unwrap_or_else(|e| exit_with_error(&e));
    let output = months
        .map(|month| format!("{}", month))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);
}

fn months(now: NaiveDate, opt: &Opt) -> Result<impl Iterator<Item = Month>, Error> {
    let year = opt.year.unwrap_or_else(|| now.year());
    if year < 1 || year > 9999 {
        return Err(Error::InvalidYear(year));
    }

    let month_number = opt.month.unwrap_or_else(|| now.month());
    if month_number < 1 || month_number > 12 {
        return Err(Error::InvalidMonth(month_number));
    }

    let m = Month::new(year, month_number).expect("invalid time");
    let mut gen = MonthGenerator::new(m);

    if opt.months_before > 0 {
        gen.nth_prev(usize::from(opt.months_before) - 1);
    }

    Ok(gen.take(usize::from(opt.months_after + opt.months_before) + 1))
}

fn exit_with_error(err: &Error) -> ! {
    eprintln!("calr: {}", err);

    let code = match err {
        Error::InvalidYear(_) | Error::InvalidMonth(_) => 64,
    };

    std::process::exit(code);
}
