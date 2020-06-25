# calr

calr is a command line tool that displays a calendar, similar to `cal(1)`.

Its output looks like:

```
      May 2020
Su Mo Tu We Th Fr Sa
                1  2
 3  4  5  6  7  8  9
10 11 12 13 14 15 16
17 18 19 20 21 22 23
24 25 26 27 28 29 30
31
```

## Installation

Currently, pre-compiled binaries of calr aren't being distributed. You can
install it with
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) by
running

```
cargo install --git https://github.com/rsookram/calr
```

## Usage

```
USAGE:
    calr [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -y <year>                 Display the specified year [default: current]
    -m <month>                Display the specified month [default: current]
    -A <months after>         Display the number of months after the current month [default: 0]
    -B <months before>        Display the number of months before the current month [default: 0]
```

## Building

calr can be built from source by cloning this repository and using Cargo.

```
git clone https://github.com/rsookram/calr
cd calr
cargo build --release
```

## Dependencies

[`pico-args`](https://crates.io/crates/pico-args) is used to parse command line
arguments and [`chrono`](https://crates.io/crates/chrono) is used for working
with time.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
