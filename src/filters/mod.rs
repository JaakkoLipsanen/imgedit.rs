pub mod parse;
mod implementations;
mod kernel;

use super::image;

/// Enum containing all the filters that the application supports
#[derive(Debug, PartialEq)]
pub enum Filter {
    Identity,
    InvertColor,
    Greyscale,
    Brighten { amount: f32 },
    Blur
}

/// Applies all filters given as parameter to the image
pub fn apply_filters(filters: &[Filter], image: &image::Image) -> Result<image::Image, Box<std::error::Error>> {
    let mut processed_image = image.clone();
    for filter in filters {
        processed_image = apply_filter(filter, &processed_image)?;
    }

    Ok(processed_image)
}

fn apply_filter(filter: &Filter, image: &image::Image) -> Result<image::Image, Box<std::error::Error>> {
    match filter {
        Filter::Identity => Ok(image.clone()),
        Filter::InvertColor => Ok(implementations::invert_color(&image)),
        Filter::Greyscale => Ok(implementations::greyscale(&image)),
        Filter::Brighten { amount } => Ok(implementations::brighten(&image, *amount)),
        Filter::Blur => Ok(implementations::blur(&image))
    }
}


