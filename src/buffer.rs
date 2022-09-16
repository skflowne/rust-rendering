pub struct ColorBuffer {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl ColorBuffer {
    pub fn new(width: usize, height: usize) -> ColorBuffer {
        ColorBuffer {
            width,
            height,
            pixels: vec![0; width * height * 4],
        }
    }

    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color)
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let pixel_data = color.to_le_bytes();
        let pixel_index = ((self.width * y) + x) * 4;

        self.pixels[pixel_index..pixel_index + 4].copy_from_slice(&pixel_data);
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}
