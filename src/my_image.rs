#![allow(dead_code)]

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
            pixels: Vec::with_capacity((width * height) as usize),
        };
    }

	/// Gets a reference to the bytes of a pixel. The length of the slice will be equal to the stride of the image.
    pub fn get_pixel(&self, x: usize, y: usize) -> &[u8] {
        let index = (((y * self.width) + x) * self.stride) as usize;

        return &self.pixels[index..index + self.stride];
    }

	/// Gets a mutable reference to the bytes of a pixel. The length of the slice will be equal to the stride of the image.
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut [u8] {
        let index = (((y * self.width) + x) * self.stride) as usize;

        return &mut self.pixels[index..index + self.stride];
    }
}
