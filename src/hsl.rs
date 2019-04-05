use num::abs;
use lodepng::{RGB};

/// Hue-Saturation-Lightness color model (alternative to RGB)
/// https://en.wikipedia.org/wiki/HSL_and_HSV
#[derive(Debug)]
pub struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

fn real_modulo (a: f32, b: f32) -> f32 { ((a % b) + b) % b }
impl HSL {
    /// based on https://en.wikipedia.org/wiki/HSL_and_HSV#General_approach
    pub fn from_rgb(rgb: RGB<u8>) -> HSL {
        let r_ = f32::from(rgb.r) / 255.0;
        let g_ = f32::from(rgb.g) / 255.0;
        let b_ = f32::from(rgb.b) / 255.0;


        let c_max = r_.max(g_.max(b_));
        let c_min = r_.min(g_.min(b_));

        let delta = c_max - c_min;
        let hue = {
            if delta == 0.0 {
                0.0
            } else if c_max == r_ {
                real_modulo((g_ - b_) / delta,6.0)
            } else if c_max == g_ {
                (b_ - r_) / delta + 2.0
            } else /* c_max == b_ */ {
                (r_ - g_) / delta + 4.0
            }
        } * 60.0;

        let lightness = (c_max + c_min) / 2.0;
        let saturation = {
            if lightness == 0.0 || lightness == 1.0 {
                0.0
            } else {
                delta / (1.0 - abs(2.0 * lightness - 1.0))
            }
        };

        HSL { h: hue, s: saturation, l: lightness }
    }

    /// based on https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB
    pub fn to_rgb(&self) -> RGB<u8> {
        let chroma = (1.0 - abs(2.0 * self.l - 1.0)) * self.s;
        let h_ = self.h / 60.0;
        let x = chroma * (1.0 - abs(h_ % 2.0 - 1.0));
        let m = self.l - chroma / 2.0;


        let rgba_f32 = match h_.floor().max(0.0).min(5.999) as i32 {
            0 => RGB::<f32> { r: chroma + m, g: x + m, b: m },
            1 => RGB::<f32> { r: x + m, g: chroma + m, b: m },
            2 => RGB::<f32> { r: m, g: chroma + m, b: x + m },
            3 => RGB::<f32> { r: m, g: x + m, b: chroma + m },
            4 => RGB::<f32> { r: x + m, g: m, b: chroma + m },
            _ /* 5 */ => RGB::<f32> { r: chroma + m, g: m, b: x + m }
        };

        RGB {
            r: (rgba_f32.r.min(1.0) * 255.0) as u8,
            g: (rgba_f32.g.min(1.0) * 255.0) as u8,
            b: (rgba_f32.b.min(1.0) * 255.0) as u8
        }
    }
}