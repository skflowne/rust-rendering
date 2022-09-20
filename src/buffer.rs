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

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let pixel_data = color.to_le_bytes();
        let pixel_index = ((self.width * y) + x) * 4;

        self.pixels[pixel_index..pixel_index + 4].copy_from_slice(&pixel_data);
    }

    /*pub fn set_pixel_n(&mut self, n: usize, color: u32) {
        let index = n * 4;
        if index >= self.pixels.len() {
            return;
        }
        self.pixels[index..index + 4].copy_from_slice(&color.to_le_bytes());
    }*/

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

impl ClearColor for ColorBuffer {
    fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color)
            }
        }
    }
}

impl Drawable for ColorBuffer {
    fn draw_grid(&mut self, spacing: usize, color: Option<u32>) {
        for y in (spacing..self.height).step_by(spacing) {
            for x in (spacing..self.width).step_by(spacing) {
                self.set_pixel(x, y, color.unwrap_or(0x00000000));
            }
        }
    }

    fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        if x >= self.width
            || y >= self.height
            || x + width >= self.width
            || y + height >= self.height
        {
            return;
        }

        for ry in y..y + height {
            for rx in x..x + width {
                self.set_pixel(rx, ry, color)
            }
        }
    }

    fn draw_point(&mut self, x: usize, y: usize, color: u32) {
        self.set_pixel(x, y, color);
    }

    fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: u32) {
        let dx = (x1 - x0) as isize;
        let dy = (y1 - y0) as isize;

        let run_length = if dx.abs() >= dy.abs() {
            dx.abs()
        } else {
            dy.abs()
        };

        let inc_x = dx as f64 / run_length as f64;
        let inc_y = dy as f64 / run_length as f64;

        let mut cx = x0 as f64;
        let mut cy = y0 as f64;
        for _ in 0..=run_length {
            self.set_pixel(cx.round() as usize, cy.round() as usize, color);
            cx += inc_x;
            cy += inc_y;
        }
    }
}

pub trait Drawable {
    fn draw_grid(&mut self, spacing: usize, color: Option<u32>);
    fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32);
    fn draw_point(&mut self, x: usize, y: usize, color: u32);
    fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: u32);
}

pub trait ClearColor {
    fn clear(&mut self, color: u32);
}

pub trait ClearAuto {
    fn clear(&mut self);
}
