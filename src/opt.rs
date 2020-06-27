use pico_args::Arguments;
use pico_args::Error;
use std::process;

/// Contains parsed command line arguments
#[derive(Debug)]
pub struct Opt {
    /// The year to display
    pub year: Option<i32>,

    /// The month to display
    pub month: Option<u32>,

    /// The number of months after the current month to display
    pub months_after: u16,

    /// The number of months before the current month to display
    pub months_before: u16,
}

impl Opt {
    /// Gets [Opt] from the command line arguments. Prints the error message
    /// and quits the program in case of failure.
    pub fn from_args() -> Self {
        let mut args = Arguments::from_env();

        if args.contains(["-h", "--help"]) {
            print_help();
            process::exit(0);
        }

        if args.contains(["-V", "--version"]) {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }

        Self::parse(args).unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1);
        })
    }

    fn parse(mut args: Arguments) -> Result<Self, Error> {
        let result = Self {
            year: args.opt_value_from_str("-y")?,
            month: args.opt_value_from_str("-m")?,
            months_after: args.opt_value_from_str("-A")?.unwrap_or(0),
            months_before: args.opt_value_from_str("-B")?.unwrap_or(0),
        };

        args.finish()?;

        Ok(result)
    }
}

fn print_help() {
    println!(
        r#"{name} {version}
Command-line tool which displays a calendar

USAGE:
    {name} [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -y <year>                 Display the specified year [default: current]
    -m <month>                Display the specified month [default: current]
    -A <months after>         Display the number of months after the current month [default: 0]
    -B <months before>        Display the number of months before the current month [default: 0]"#,
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
    );
}
