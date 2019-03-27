use super::image;
use num;
use lodepng;
use lodepng::{RGBA};

pub struct Kernel3x3 {
    pub matrix: [f32; 3 * 3]
}

pub struct Kernel5x5 {
    pub matrix: [f32; 5 * 5]
}

pub trait Kernel {
    fn get_slice(&self) -> &[f32];
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

impl Kernel for Kernel3x3 {
    fn get_slice(&self) -> &[f32] { &self.matrix }
    fn get_width(&self) -> usize { 3 }
    fn get_height(&self) -> usize { 3 }
}

impl Kernel for Kernel5x5 {
    fn get_slice(&self) -> &[f32] { &self.matrix }
    fn get_width(&self) -> usize { 5 }
    fn get_height(&self) -> usize { 5 }
}

pub fn apply_kernel(kernel: &Kernel, image: &image::Image) -> image::Image {
    let mut modified_image = image.clone();
    for x in 0..image.width {
        for y in 0..image.height {
            let i =  y * image.width + x;
            modified_image.buffer[i] = apply_kernel_for_pixel(kernel, (x, y), image);
        }
    }

    modified_image
}


fn apply_kernel_for_pixel(kernel: &Kernel, pixel_xy: (usize, usize), image: &image::Image) -> RGBA {
    let mut accum = (0.0, 0.0, 0.0, 0.0);

    let kernel_data = kernel.get_slice();
    let w = kernel.get_width();
    let h  = kernel.get_height();
    for i in 0..w {
        for j in 0..h {
            let px = num::clamp((pixel_xy.0 + i) as i64 - (w / 2) as i64, 0, image.width as i64 - 1) as usize;
            let py = num::clamp((pixel_xy.1 + j) as i64 - (h / 2) as i64, 0, image.height as i64 - 1) as usize;
            let pixel = image.buffer[py * image.width + px];
            let kernel_value = kernel_data[j * w + i];

            accum.0 += f32::from(pixel.r) * kernel_value;
            accum.1 += f32::from(pixel.g) * kernel_value;
            accum.2 += f32::from(pixel.b) * kernel_value;
            accum.3 += f32::from(pixel.a) * kernel_value;
        }
    }

    RGBA { r: clamp_to_u8(accum.0), g: clamp_to_u8(accum.1), b: clamp_to_u8(accum.2), a: clamp_to_u8(accum.3) }
}

fn clamp_to_u8(v: f32) -> u8 {
    num::clamp(v.round(), 0.0, 255.0) as u8
}