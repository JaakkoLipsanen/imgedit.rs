use super::image;
use num;

pub fn invert_color(image: &image::Image) -> image::Image {
    let mut modified = image.clone();
    for i in 0..modified.buffer.len() {
        let rgba = &mut modified.buffer[i];
        rgba.r = 255 - rgba.r;
        rgba.g = 255 - rgba.g;
        rgba.b = 255 - rgba.b;
    }

    modified
}

pub fn greyscale(image: &image::Image) -> image::Image {
    let mut modified = image.clone();
    for i in 0..modified.buffer.len() {
        let rgba = &mut modified.buffer[i];
        let color_linear = rgba.r as f32 / 255.0 * 0.2126 + rgba.g as f32 / 255.0 * 0.7152 + rgba.b as f32 / 255.0 * 0.0722;
        let clamped = num::clamp((color_linear * 255.0) as u8, 0, 255);

        rgba.r = clamped;
        rgba.g = clamped;
        rgba.b = clamped;
    }

    modified
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use lodepng::RGBA;

    fn create_test_img(pixels: &Vec<RGBA>) -> image::Image {
        image::Image {
            buffer: pixels.clone(),
            width: pixels.len(),
            height: 1
        }
    }

    #[test]
    fn test_invert_color() {
        let img = create_test_img(&vec![RGBA { r: 10, g: 66, b: 67, a: 210 }]);
        assert_eq!(invert_color(&img).buffer[0], RGBA { r: 255 - 10, g: 255 - 66, b: 255 - 67, a: 210 });
    }

    #[test]
    fn test_invert_color_twice_returns_original() {
        let img = create_test_img(&vec![RGBA { r: 10, g: 66, b: 67, a: 210 }]);
        assert_eq!(invert_color(&invert_color(&img)).buffer[0], RGBA { r: 10, g: 66, b: 67, a: 210 });
    }

    #[test]
    fn test_invert_color_doesnt_change_size() {
        let img = create_test_img(&vec![RGBA { r: 10, g: 66, b: 67, a: 210 }]);
        let inverted = &invert_color(&img);
        assert_eq!(inverted.width, img.width);
        assert_eq!(inverted.height, img.height);
    }

    fn assert_rgb_components_equal(rgba: &RGBA) {
        assert_eq!(rgba.r, rgba.g);
        assert_eq!(rgba.g, rgba.b);
    }

    #[test]
    fn test_greyscale() {
        let img = create_test_img(&vec![RGBA { r: 10, g: 66, b: 67, a: 255 }, RGBA { r: 255, g: 128, b: 0, a: 128 }]);
        let greyscale = &greyscale(&img);
        assert_rgb_components_equal(&greyscale.buffer[0]);
        assert_rgb_components_equal(&greyscale.buffer[1]);

        assert_eq!(greyscale.buffer[0].a, 255);
        assert_eq!(greyscale.buffer[1].a, 128);
    }
}