use super::image;
use super::kernel;
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
        let color_linear = f32::from(rgba.r) / 255.0 * 0.2126 + f32::from(rgba.g) / 255.0 * 0.7152 + f32::from(rgba.b) / 255.0 * 0.0722;
        let clamped = num::clamp((color_linear * 255.0) as u8, 0, 255);

        rgba.r = clamped;
        rgba.g = clamped;
        rgba.b = clamped;
    }

    modified
}

pub fn brighten(image: &image::Image, amount: f32) -> image::Image {
    let kernel = kernel::Kernel3x3 {
        matrix: [
            0.0, 0.0, 0.0,
            0.0, 1.0 + amount, 0.0,
            0.0, 0.0, 0.0
        ]
    };

    kernel::apply_kernel(&kernel, &image)
}

pub fn blur(image: &image::Image) -> image::Image {
    let kernel = kernel::Kernel5x5 {
        matrix: [
            0.003_765,	0.015_019,	0.023_792,	0.015_019,	0.003_765,
            0.015_019,	0.059_912,	0.094_907,	0.059_912,	0.015_019,
            0.023_792,	0.094_907,	0.150_342,	0.094_907,	0.023_792,
            0.015_019,	0.059_912,	0.094_907,	0.059_912,	0.015_019,
            0.003_765,	0.015_019,	0.023_792,	0.015_019,	0.003_765,
        ]
    };

    kernel::apply_kernel(&kernel, &image)
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

    #[test]
    fn test_brighten() {
        let img = create_test_img(&vec![RGBA { r: 10, g: 66, b: 67, a: 255 }]);
        let brightened = &brighten(&img, 1.0);

        assert_eq!(brightened.buffer[0].r, 10 * 2);
        assert_eq!(brightened.buffer[0].g, 66 * 2);
        assert_eq!(brightened.buffer[0].b, 67 * 2);
        assert_eq!(brightened.buffer[0].a, 255);
    }

    #[test]
    fn test_brighten_zero_does_not_modify() {
        let img = create_test_img(&vec![RGBA { r: 10, g: 66, b: 67, a: 255 }]);
        let brightened = &brighten(&img, 0.0);

        assert_eq!(brightened.buffer[0].r, 10);
        assert_eq!(brightened.buffer[0].g, 66);
        assert_eq!(brightened.buffer[0].b, 67);
        assert_eq!(brightened.buffer[0].a, 255);
    }

    #[test]
    fn test_blur() {
        let img = create_test_img(&vec![
            RGBA { r: 255, g: 0, b: 255, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 255, g: 0, b: 255, a: 255 },
            RGBA { r: 255, g: 0, b: 255, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 255, g: 0, b: 255, a: 255 },
            RGBA { r: 255, g: 0, b: 255, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 255, g: 0, b: 255, a: 255 },
            RGBA { r: 255, g: 0, b: 255, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 255, g: 0, b: 255, a: 255 },
            RGBA { r: 255, g: 0, b: 255, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 128, g: 128, b: 128, a: 255 }, RGBA { r: 255, g: 0, b: 255, a: 255 },
        ]);
        let brightened = &blur(&img);

        let center_index = 2 * 5 + 2;

        // make sure that values are changed towards the average of the nearest values
        assert!(brightened.buffer[center_index].r > 128);
        assert!(brightened.buffer[center_index].g < 128);
        assert!(brightened.buffer[center_index].b > 128);
        assert_eq!(brightened.buffer[center_index].a, 255);
    }
}