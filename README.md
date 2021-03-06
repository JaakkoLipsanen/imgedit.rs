[![codecov](https://codecov.io/gh/JaakkoLipsanen/imgedit.rs/branch/master/graph/badge.svg)](https://codecov.io/gh/JaakkoLipsanen/imgedit.rs)

# imgedit.rs
CLI image manipulation tool written in Rust for University of Helsinki's Data Structures Project -course.

[Documentation](/docs)  
[Code coverage](https://codecov.io/gh/JaakkoLipsanen/imgedit.rs)

## How to run
### Binary
OSX build can be found in [releases](https://github.com/JaakkoLipsanen/imgedit.rs/releases/tag/final-return)

### Build yourself
1. [Install Rust and Cargo](https://www.rust-lang.org/tools/install) (**NOTE! Nightly version of Rust required** for benchmark tests)
2. Run with `cargo run -- --input <INPUT IMAGE PATH> --output <OUTPUT PATH> --apply <LIST OF FILTERS HERE>`  
Note: Only supported image format is png.

Available filters are:
- `identity` - does nothing
- `invert-color` - inverts color of each pixel
- `greyscale` - transforms image into greyscale
- `blur` - applies a simple 5px wide blur on the image
- `unsharp-mask` - sharpens the image with [unsharp masking technique](https://en.wikipedia.org/wiki/Unsharp_masking)
- `hue-shift=<amount>`, where `amount` is between [0, 360] - shifts the [hue](https://en.wikipedia.org/wiki/Hue) of the each pixel of in the image by `amount`.
- `brighten=<amount>`, where `amount` is between [0, 1] - brightens each pixel by `amount`, where `amount = 1` means that each pixels every component is multiplied by two.

So an example command could be: `cargo run -- --input input.png --output output.png --apply invert-color hue-shift=180 blur`
