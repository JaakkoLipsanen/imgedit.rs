use lodepng::RGBA;
use std::path::{Path};

/// Struct containing the Image information
#[derive(Clone)]
pub struct Image {
    pub buffer: Vec<RGBA>,
    pub width: usize,
    pub height: usize,
}

/// Loads the PNG image from the given path
pub fn load_image(path: &Path) -> Result<Image, Box<std::error::Error>> {
    if !path.exists() {
        bail!("File '{}' doesn't exist", path.to_str().unwrap())
    }

    match lodepng::decode32_file(path) {
        Ok(image) => Ok(Image { buffer: image.buffer.to_vec(), width: image.width, height: image.height }),
        Err(_e) => bail!("File '{}' doesn't seem to be a PNG", path.to_str().unwrap())
    }
}

/// Saves the image as PNG to the given path
pub fn save_image(image: &Image, path: &Path) -> Result<(), Box<std::error::Error>> {
    lodepng::encode32_file(path, &image.buffer, image.width, image.height)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_image_fails_on_invalid_path() {
        assert!(load_image(Path::new("non-existing-image.png")).is_err(), "Non existing image should have returned Err");
    }

    #[test]
    fn test_load_image_succeeds_on_valid_path() {
        assert!(load_image(Path::new("example_inputs/outside.png")).is_ok(), "Existing image should have returned Ok");
    }

    #[test]
    fn test_load_image_detects_size_correctly() {
        let  image = load_image(Path::new("example_inputs/outside.png")).unwrap();
        assert_eq!(image.width, 800, "Image width was detected incorrectly");
        assert_eq!(image.height, 529, "Image height was detected incorrectly");

    }
}