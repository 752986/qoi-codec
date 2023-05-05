#![allow(dead_code)]

use std::io;

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
            pixels: Vec::with_capacity(width * height),
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
            pixels
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

    /// Read the qoi image at `filepath` into an `Image`. Returns `Err` if the file can't be opened for some reason.
    pub fn read_qoi(filepath: &str) -> io::Result<Self> {
        use image::io::Reader;
        let image = Reader::open(filepath)?.decode().unwrap();
        let samples = image.as_flat_samples_u8().unwrap();

        let pixels = samples.as_slice().to_owned();
        let width = samples.layout.width as usize;
        let stride = samples.layout.width_stride;

        return Ok(Image::from(pixels, width, stride));
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
