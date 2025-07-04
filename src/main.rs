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
        .map(|month| month.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{output}");
}

fn months(now: Date, opt: &Opt) -> Result<impl Iterator<Item = Month>, Error> {
    let year = match opt.year {
        Some(y) => {
            if !(1..=9999).contains(&y) {
                return Err(Error::InvalidYear(y));
            }

            y as i32
        }
        None => now.year(),
    };

    let month_number = match opt.month {
        Some(num) => {
            if !(1..=12).contains(&num) {
                return Err(Error::InvalidMonth(num));
            }

            num as u8
        }
        None => u8::from(now.month()),
    };

    let mut m = Month::new(year, month_number).expect("valid time");
    for _ in 0..opt.months_before {
        m = m.prev().expect("valid time");
    }

    let generator = MonthGenerator::new(m);

    Ok(generator.take(usize::from(opt.months_after) + usize::from(opt.months_before) + 1))
}

fn exit_with_error(err: &Error) -> ! {
    eprintln!("calr: {err}");

    let code = match err {
        Error::InvalidYear(_) | Error::InvalidMonth(_) => 64,
        Error::UnknownOffset => 1,
    };

    std::process::exit(code);
}
