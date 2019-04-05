#[macro_use] extern crate simple_error;
extern crate structopt;

mod filters;
mod image;
mod hsl;

use std::path::{PathBuf};
use structopt::StructOpt;

/// Struct containing all the command line arguments for the application
#[derive(StructOpt, Debug)]
struct CliArgs {
    /// The input image path
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    pub input: PathBuf,

    /// The output image path where the modified input image will be saved
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output: PathBuf,

    /// All the filters that are going to be applied to the input image, as strings
    // todo: would be nice if type was Vec<Filter> and Filter was enum, so that
    // there would be an structopts generated error for invalid filters
    #[structopt(short = "a", long = "apply")]
    pub filters: Vec<String>,
}

/// The main flow of the application is located here
fn run() -> Result<(), Box<std::error::Error>> {
    let opts: CliArgs = CliArgs::from_args();
    let filters = filters::parse::parse_filters(&opts.filters)?;
    let image  = image::load_image(&opts.input)?;

    let processed_image = filters::apply_filters(&filters, &image)?;
    image::save_image(&processed_image, &opts.output)?;

    println!("Output saved to {}", opts.output.to_str().unwrap());
    Ok(())
}

/// Entry point for the application
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.description())
    }
}