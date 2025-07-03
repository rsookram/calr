use std::{
    env::args_os,
    ffi::{OsStr, OsString},
    process,
};

/// Contains parsed command line arguments
#[derive(Debug)]
pub struct Opt {
    /// The year to display
    pub year: Option<u16>,

    /// The month to display
    pub month: Option<u16>,

    /// The number of months after the current month to display
    pub months_after: u16,

    /// The number of months before the current month to display
    pub months_before: u16,
}

impl Opt {
    /// Gets [Opt] from the command line arguments. Prints the error message
    /// and quits the program in case of failure.
    pub fn from_args() -> Self {
        let args = Arguments::from_env();

        if args.contains("-h") || args.contains("--help") {
            print_usage();
            process::exit(0);
        }

        if args.contains("-V") || args.contains("--version") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }

        Self::parse(args).unwrap_or_else(|e| {
            eprintln!("error: {e}");
            process::exit(1);
        })
    }

    fn parse(args: Arguments) -> Result<Self, String> {
        let result = Self {
            year: args.opt_value_as_u16("-y")?,
            month: args.opt_value_as_u16("-m")?,
            // Default to 0 months before / after only when the options aren't set. Error out if
            // they fail to parse.
            months_after: args.opt_value_as_u16("-A")?.unwrap_or(0),
            months_before: args.opt_value_as_u16("-B")?.unwrap_or(0),
        };

        Ok(result)
    }
}

fn print_usage() {
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

/// The raw command line arguments
#[derive(Debug)]
struct Arguments {
    /// args contains all the command line arguments following the program name
    args: Vec<OsString>,
}

impl Arguments {
    fn from_env() -> Self {
        Self {
            args: args_os().skip(1).collect(),
        }
    }

    /// Returns whether there's an argument matching the given `key`
    fn contains(&self, key: &'static str) -> bool {
        self.args.iter().any(|arg| arg == key)
    }

    /// Returns the command line argument following the given `key`
    fn opt_os_str(&self, key: &'static str) -> Option<&OsStr> {
        let idx = self.args.iter().position(|arg| arg == key)?;
        Some(self.args.get(idx + 1)?)
    }

    fn opt_value_as_u16(&self, key: &'static str) -> Result<Option<u16>, String> {
        let Some(value_str) = self.opt_os_str(key) else {
            return Ok(None);
        };

        let str = value_str
            .to_str()
            .ok_or_else(|| format!("invalid argument for '{key}' {}", value_str.display()))?;
        Ok(Some(str.parse::<u16>().map_err(|err| {
            format!("failed to parse value '{str}' for key '{key}': {err}")
        })?))
    }
}
