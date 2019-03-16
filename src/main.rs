extern crate structopt;
#[macro_use]
extern crate simple_error;

use std::path::PathBuf;
use structopt::StructOpt;
use std::error::Error;
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

fn read_args() -> CliArgs {
    return CliArgs::from_args();
}

#[derive(Debug)]
enum Filter {
    Identity,
    InvertColor,
    Brighten { amount: f32 }
}

fn main() {
    let opts: CliArgs = CliArgs::from_args();
    let filters = match parse_filters(opts.filters) {
        Ok(filters) => filters,
        Err(err) => {
            eprintln!("{}", err.to_string());
            return;
        }
    };

    let input = match File::open(opts.input) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Error opening input file. It probably doesn't exist");
            return;
        }
    };

    println!("Hello, world! {:?}", filters);
}

fn parse_filters(filters: Vec<String>) -> Result<Vec<Filter>, Box<Error>> {
    return filters
        .into_iter()
        .map(|filter| {
            let filter_name: Vec<&str> = filter.split("=").collect();
            let args: Option<Vec<&str>> = filter_name.get(1).map(|args| args.split(",").collect());

            return match filter_name.first() {
                Some(&"identity") => Ok(Filter::Identity),
                Some(&"invert-color") => Ok(Filter::InvertColor),
                Some(&"brighten") => {
                    match args {
                        Some(args) => {
                            if args.len() != 1 {
                                bail!("Invalid amount of arguments for brighten filter. Only one argument is needd")
                            }

                            match args.first().unwrap().parse::<f32>() {
                                Ok(amount) => Ok(Filter::Brighten { amount }),
                                Err(err) => bail!("Invalid parameter supplied for brighten filter: must be a number")
                            }
                        },
                        None => bail!("Missing argument in brighten filter. Use: 'brighten=0.5'")
                    }

                }
                _ => bail!("Invalid filter argument '{}'", filter)
            }
        })
        .collect();
}
