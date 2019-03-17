extern crate structopt;
#[macro_use]
extern crate simple_error;

mod filters;
mod image;

use std::path::{PathBuf};
use structopt::StructOpt;
use crate::filters::Filter;

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

// TODO: implement
fn apply_filters<'a>(image: &'a mut image::Image, _filters: &Vec<Filter>) -> Result<&'a image::Image, Box<std::error::Error>> {
    Ok(image)
}

fn run() -> Result<(), Box<std::error::Error>> {
    let opts: CliArgs = CliArgs::from_args();
    let filters = filters::parse_filters(opts.filters)?;
    let mut image  = image::load_image(&opts.input)?;

    apply_filters(&mut image, &filters)?;
    image::save_image(&image, &opts.output)?;

    println!("Output saved to {}", opts.output.to_str().unwrap());
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e.description())
    }
}