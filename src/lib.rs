use std::time::Duration;

use buffer::ColorBuffer;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, Texture},
    video::Window,
    EventPump,
};

mod buffer;

pub fn run() {
    let mut ctx = setup();

    main_loop(&mut ctx);
}

fn setup() -> WindowCtx {
    WindowCtx::build("3dRendererer", 800, 600)
}

fn main_loop(ctx: &mut WindowCtx) {
    let texture_creator = ctx.canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            PixelFormatEnum::ARGB8888,
            sdl2::render::TextureAccess::Streaming,
            800,
            600,
        )
        .unwrap();

    let mut running = true;
    while running {
        running = ctx.process_input();
        ctx.render(&mut texture);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

struct WindowCtx {
    width: usize,
    height: usize,
    canvas: Canvas<Window>,
    color_buffer: ColorBuffer,
    event_pump: EventPump,
}

impl WindowCtx {
    fn build(title: &str, mut width: u32, mut height: u32) -> WindowCtx {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();

        match video.display_mode(0, 0) {
            Ok(mode) => {
                width = mode.w as u32;
                height = mode.h as u32;
                println!("Display mode: {:?}", mode);
            }
            Err(e) => eprintln!(
                "Failed to get display mode: {}, using default width and height",
                e
            ),
        };

        let window = video
            .window(title, width, height)
            .borderless()
            .position_centered()
            //.fullscreen()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        let color_buffer = ColorBuffer::new(width as usize, height as usize);

        let event_pump = ctx.event_pump().unwrap();

        println!("WindowCtx w: {} h: {}", width, height);

        WindowCtx {
            width: width as usize,
            height: height as usize,
            canvas,
            color_buffer,
            event_pump,
        }
    }

    fn process_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Received quit event, shutting down");
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn draw_grid(&mut self, spacing: usize, color: Option<u32>) {
        for y in (spacing..self.height).step_by(spacing) {
            for x in (spacing..self.width).step_by(spacing) {
                self.color_buffer
                    .set_pixel(x, y, color.unwrap_or(0x00000000));
            }
        }
    }

    fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        /*let x = if x >= self.width { self.width - 1 } else { x };
        let y = if y >= self.height { self.height - 1 } else { y };
        let width = if x + width >= self.width {
            self.width - x
        } else {
            width
        };
        let height = if y + height >= self.height {
            self.height - 1
        } else {
            height
        };*/

        println!("Draw rect x:{} y:{} w:{} h:{}", x, y, width, height);

        for ry in 0..self.height {
            for rx in 0..self.width {
                if rx >= x && rx <= x + width && ry >= y && ry <= y + height {
                    self.color_buffer.set_pixel(rx, ry, color);
                }
            }
        }
        /*
        for ry in y..y + height {
            for rx in x..x + width {
                self.color_buffer.set_pixel(rx, ry, color)
            }
        }*/
    }

    fn render(&mut self, texture: &mut Texture) {
        self.color_buffer.clear(0xFFFF00);
        self.draw_grid(10, Some(0xFF999999));
        self.draw_rect(10, 10, 10, self.height - 20, 0xFFFF0000);

        self.copy_buffer_to_canvas(texture);

        self.canvas.present();
    }

    fn copy_buffer_to_canvas(&mut self, texture: &mut Texture) {
        texture
            .update(None, self.color_buffer.pixel_data(), self.width * 4)
            .unwrap();
        self.canvas.copy(texture, None, None).unwrap();
    }
}
