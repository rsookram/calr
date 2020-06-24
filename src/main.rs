mod error;
mod opt;

use calr::month::iter::MonthGenerator;
use calr::month::Month;
use chrono::prelude::*;
use error::Error;
use gumdrop::Options;
use opt::Opt;

fn main() {
    let opt = Opt::parse_args_default_or_exit();

    if opt.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }

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
