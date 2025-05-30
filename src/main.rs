mod error;
mod opt;

use calr::month::iter::MonthGenerator;
use calr::month::Month;
use error::Error;
use opt::Opt;
use time::Date;
use time::OffsetDateTime;

fn main() {
    let opt = Opt::from_args();

    let now = match OffsetDateTime::now_local() {
        Ok(dt) => dt.date(),
        Err(_) => exit_with_error(&Error::UnknownOffset),
    };

    let months = months(now, &opt).unwrap_or_else(|e| exit_with_error(&e));
    let output = months
        .map(|month| format!("{month}"))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{output}");
}

fn months(now: Date, opt: &Opt) -> Result<impl Iterator<Item = Month>, Error> {
    let year = opt.year.unwrap_or_else(|| now.year());
    if !(1..=9999).contains(&year) {
        return Err(Error::InvalidYear(year));
    }

    let month_number = opt.month.unwrap_or_else(|| u8::from(now.month()));
    if !(1..=12).contains(&month_number) {
        return Err(Error::InvalidMonth(month_number));
    }

    let m = Month::new(year, month_number).expect("invalid time");
    let mut generator = MonthGenerator::new(m);

    if opt.months_before > 0 {
        generator.nth_prev(usize::from(opt.months_before) - 1);
    }

    Ok(generator.take(usize::from(opt.months_after + opt.months_before) + 1))
}

fn exit_with_error(err: &Error) -> ! {
    eprintln!("calr: {err}");

    let code = match err {
        Error::InvalidYear(_) | Error::InvalidMonth(_) => 64,
        Error::UnknownOffset => 1,
    };

    std::process::exit(code);
}
