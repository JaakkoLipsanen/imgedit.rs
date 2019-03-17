#[macro_use] extern crate simple_error;
extern crate structopt;


mod filters;
mod image;

use std::path::{PathBuf};
use structopt::StructOpt;
use crate::filters::Filter;

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

/// Applies all filters given as parameter to the image
fn apply_filters<'a>(image: &'a mut image::Image, _filters: &[Filter]) -> Result<&'a image::Image, Box<std::error::Error>> {
    Ok(image)
}

/// The main flow of the application is located here
fn run() -> Result<(), Box<std::error::Error>> {
    let opts: CliArgs = CliArgs::from_args();
    let filters = filters::parse_filters(opts.filters)?;
    let mut image  = image::load_image(&opts.input)?;

    apply_filters(&mut image, &filters)?;
    image::save_image(&image, &opts.output)?;

    println!("Output saved to {}", opts.output.to_str().unwrap());
    Ok(())
}

/// Entry point for the application
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.description())
    }
}