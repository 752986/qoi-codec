#![allow(dead_code)]

use std::{
    error::Error,
    io::{self, Read}, fmt::Display,
};

#[derive(Debug)]
pub struct ImageDecodeError {
    position: usize,
    message: String,
}

impl Display for ImageDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error decoding image at byte {}: {}", self.position, self.message)
    }
}

impl Error for ImageDecodeError {}

// impl Into<io::Error> for ImageDecodeError {
//     fn into(self) -> io::Error {
//         io::Error::new(kind, error)
//     }
// }

/// Contains the data of an image.
pub struct Image {
    /// The width of the image.
    pub width: usize,
    /// The height of the image.
    pub height: usize,
    /// The number of channels per pixel (3 for RGB, 4 for RGBA).
    stride: usize,
    /// The underlying pixel data of the image.
    pixels: Vec<u8>,
}

impl Image {
    /// Creates a new empty image. `stride` is the number of channels per pixel (3 for RGB, 4 for RGBA).
    pub fn new(width: usize, height: usize, stride: usize) -> Self {
        return Self {
            width,
            height,
            stride,
            pixels: Vec::with_capacity(width * height * stride),
        };
    }

    /// Creates a new image from a pre-existing pixel array. `stride` is the number of channels per pixel (3 for RGB, 4 for RGBA).
    pub fn from(pixels: Vec<u8>, width: usize, stride: usize) -> Self {
        let n_pixels = pixels.len() / stride;
        let height = n_pixels / width;
        return Self {
            width,
            height,
            stride,
            pixels,
        };
    }

    /// Read the png image at `filepath` into an `Image`. Returns `Err` if the file can't be opened for some reason.
    pub fn read_png(filepath: &str) -> io::Result<Self> {
        use image::io::Reader;
        let image = Reader::open(filepath)?.decode().unwrap();
        let samples = image.as_flat_samples_u8().unwrap();

        let pixels = samples.as_slice().to_owned();
        let width = samples.layout.width as usize;
        let stride = samples.layout.width_stride;

        return Ok(Image::from(pixels, width, stride));
    }

    // fn decode_qoi(data: &[u8]) -> Result<Self, ImageDecodeError> {
    //     if
    //     // let pixels =
    //     // let width =
    //     // let stride =
    //
    //     return Ok(Image::from(pixels, width, stride));
    // }

    /// Read the qoi image at `filepath` into an `Image`. Returns `Err` if the file can't be opened for some reason.
    pub fn read_qoi(filepath: &str) -> io::Result<Self> {
        #[derive(Clone, Copy)]
        struct Color {
            r: u8,
            g: u8,
            b: u8,
            a: u8
        }

        impl Color {
            fn get_hash(&self) -> u8 {
                return (self.r * 3 + self.g * 5 + self.b * 7 + self.a * 11) % 64;
            }
        }

        use std::fs::File;
        let mut file = File::open(filepath)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        // check magic bytes "qoif"
        if &data[0..4] != b"qoif" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a QOI image file"));
        }

        let width = u32::from_be_bytes(data[4..8].try_into().unwrap());
        let height = u32::from_be_bytes(data[8..12].try_into().unwrap());
        let n_channels = data[12];
        if n_channels != 3 && n_channels != 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid number of channels"));
        }

        let prev_color = Color { r: 0, b: 0, g: 0, a: 255 };
        let prev_colors = [Color { r: 0, b: 0, g: 0, a: 255 }; 64];
        let result = Image::new(width as usize, height as usize, n_channels as usize);

        let current_byte: usize = 14;
        loop {
            let computed_color: Color = match data[current_byte] {
                0b11111110 => { // RGB full value
                    Color {
                        r: data[current_byte + 1],
                        b: data[current_byte + 2],
                        g: data[current_byte + 3],
                        a: prev_color.a
                    }
                }
                0b11111111 => { // RGBA full value
                    Color {
                        r: data[current_byte + 1],
                        b: data[current_byte + 2],
                        g: data[current_byte + 3],
                        a: data[current_byte + 4]
                    }
                }
                byte => match byte >> 6 {
                    0b00 => { // index lookup
                        prev_colors[(byte & 0b00111111) as usize]
                    }
                    0b01 => { // RGB diff
                        let dr = ((byte >> 4) & 0b11) - 2;
                        let dg = ((byte >> 2) & 0b11) - 2;
                        let db = ((byte >> 0) & 0b11) - 2;
                        Color {
                            r: prev_color.r + dr,
                            g: prev_color.g + dg,
                            b: prev_color.b + db,
                            a: prev_color.a
                        }
                    }
                    0b10 => { // luma diff
                        let dg = byte & 0b00111111;
                        let dr = u8::overflowing_add(data[current_byte + 1] >> 4, dg).0;
                        let db = u8::overflowing_add(data[current_byte + 1] & 0b00001111, dg).0;
                        Color {
                            r: prev_color.r + dr,
                            g: prev_color.g + dg,
                            b: prev_color.b + db,
                            a: prev_color.a
                        }
                    }
                    0b11 => { // run
                        // for i in 0..byte & 0b00111111 {
                            
                        // }
                        todo!()
                    }
                    _ => {
                        panic!("This should never happen") // safe because of the bitshift
                    }
                }
            };
        }

        // return Ok();
        return todo!()
    }

    pub fn write_png(&self, filepath: &str) -> bool {
        return image::save_buffer_with_format(
            filepath,
            &self.pixels,
            self.width as u32,
            self.height as u32,
            if self.stride == 3 {
                image::ColorType::Rgb8
            } else {
                image::ColorType::Rgba8
            },
            image::ImageFormat::Png,
        )
        .is_ok();
    }

    /// Gets a reference to the bytes of a pixel. The length of the slice will be equal to the stride of the image.
    pub fn get_pixel(&self, x: usize, y: usize) -> &[u8] {
        let index = ((y * self.width) + x) * self.stride;

        return &self.pixels[index..index + self.stride];
    }

    /// Gets a mutable reference to the bytes of a pixel. The length of the slice will be equal to the stride of the image.
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut [u8] {
        let index = ((y * self.width) + x) * self.stride;

        return &mut self.pixels[index..index + self.stride];
    }
}
