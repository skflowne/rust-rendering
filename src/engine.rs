use std::time::Duration;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::{Canvas, Texture, UpdateTextureError},
    video::Window,
    EventPump,
};

use crate::buffer::{ClearAuto, ClearColor, ColorBuffer, Drawable};

pub struct EngineConfig {
    pub window_title: String,
    pub width: usize,
    pub height: usize,
    pub clear_color: u32,
}

type EngineUpdateFn<'a> = &'a mut dyn FnMut(&mut Engine);

pub struct Engine<'a> {
    config: EngineConfig,
    canvas: Canvas<Window>,
    color_buffer: ColorBuffer,
    event_pump: EventPump,
    update: Option<EngineUpdateFn<'a>>,
}

impl<'a> Engine<'a> {
    pub fn build(mut config: EngineConfig) -> Engine<'a> {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();

        match video.display_mode(0, 0) {
            Ok(mode) => {
                config.width = mode.w as usize;
                config.height = mode.h as usize;
                println!("Display mode: {:?}", mode);
            }
            Err(e) => eprintln!(
                "Failed to get display mode: {}, using default width and height",
                e
            ),
        };

        let width = config.width;
        let height = config.height;

        let window = video
            .window(&config.window_title, width as u32, height as u32)
            .borderless()
            .position_centered()
            .fullscreen()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let color_buffer = ColorBuffer::new(width, height);

        let event_pump = ctx.event_pump().unwrap();

        println!("WindowCtx w: {} h: {}", width, height);

        Engine {
            config,
            canvas,
            color_buffer,
            event_pump,
            update: None,
        }
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
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

    pub fn on_update(&mut self, update: EngineUpdateFn<'a>) {
        self.update = Some(update);
        self.update();
    }

    fn custom_update(&mut self) {
        let f: *mut EngineUpdateFn<'a> = self.update.as_mut().unwrap();
        unsafe {
            (*f)(self);
        }
        /*let up: *mut EngineUpdateFn = self.update.unwrap();
        unsafe {
            ((*up).update_fn)(self);
        }*/
    }

    fn update(&mut self) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture(
                PixelFormatEnum::ARGB8888,
                sdl2::render::TextureAccess::Streaming,
                self.config.width as u32,
                self.config.height as u32,
            )
            .unwrap();

        let mut running = true;
        while running {
            running = self.process_input();

            self.custom_update();
            self.render_buffer(&mut texture).unwrap();
            self.clear_buffer();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn clear_buffer(&mut self) {
        self.color_buffer.clear(self.config.clear_color);
    }

    fn render_buffer(&mut self, texture: &mut Texture) -> Result<(), UpdateTextureError> {
        self.copy_buffer_to_canvas(texture)?;

        self.canvas.present();

        Ok(())
    }

    fn copy_buffer_to_canvas(&mut self, texture: &mut Texture) -> Result<(), UpdateTextureError> {
        texture.update(None, self.color_buffer.pixels(), self.config.width * 4)?;
        self.canvas.copy(texture, None, None).unwrap();

        Ok(())
    }
}

impl<'a> ClearAuto for Engine<'a> {
    fn clear(&mut self) {
        self.color_buffer.clear(self.config.clear_color);
    }
}

impl<'a> Drawable for Engine<'a> {
    fn draw_grid(&mut self, spacing: usize, color: Option<u32>) {
        self.color_buffer.draw_grid(spacing, color);
    }

    fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        self.color_buffer.draw_rect(x, y, width, height, color);
    }

    fn draw_point(&mut self, x: usize, y: usize, color: u32) {
        self.color_buffer.draw_point(x, y, color);
    }
}
