extern crate structopt;
#[macro_use]
extern crate simple_error;

mod filters;

use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::File;

#[derive(StructOpt, Debug)]
struct CliArgs {
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    pub input: PathBuf,

    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: PathBuf,

    // todo: would be nice if type was Vec<Filter> and Filter was enum, so that
    // there would be an structopts generated error for invalid filters
    #[structopt(short = "a", long = "apply")]
    pub filters: Vec<String>,
}

fn main() {
    let opts: CliArgs = CliArgs::from_args();
    let filters = match filters::parse_filters(opts.filters) {
        Ok(filters) => filters,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        }
    };

    let input = match File::open(opts.input) {
        Ok(input) => input,
        Err(_err) => {
            eprintln!("Error opening input file. It probably doesn't exist");
            return;
        }
    };

    println!("Input file {:?} exists. Filters detected: {:?}", input, filters);
}
