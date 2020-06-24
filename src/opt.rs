use gumdrop::Options;

/// Command-line tool which displays a calendar
#[derive(Debug, Options)]
pub struct Opt {
    // Options here can be accepted with any command (or none at all),
    // but they must come before the command name.
    #[options(help = "Prints help information")]
    help: bool,

    #[options(help = "Prints version information\n")]
    pub version: bool,

    /// Display the specified year [default: current]
    #[options(short = "y", no_long, meta = "<year>")]
    pub year: Option<i32>,

    /// Display the specified month [default: current]
    #[options(short = "m", no_long, meta = "<month>")]
    pub month: Option<u32>,

    /// Display the number of months after the current month [default: 0]
    #[options(short = "A", no_long, meta = "<months after>")]
    pub months_after: u16,

    /// Display the number of months before the current month [default: 0]
    #[options(short = "B", no_long, meta = "<months before>")]
    pub months_before: u16,
}
