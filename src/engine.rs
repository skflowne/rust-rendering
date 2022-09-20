use std::time::{Duration, Instant};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::{Canvas, Texture, UpdateTextureError},
    video::Window,
    EventPump,
};

use crate::buffer::{ClearAuto, ClearColor, ColorBuffer, Drawable};
use crate::utils::NumOption;

pub struct EngineConfigParams {
    pub window_title: Option<String>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub clear_color: Option<u32>,
    pub fps: Option<u32>,
}

impl Default for EngineConfigParams {
    fn default() -> Self {
        EngineConfigParams {
            window_title: None,
            width: None,
            height: None,
            clear_color: None,
            fps: None,
        }
    }
}

pub struct EngineConfig {
    window_title: String,
    width: usize,
    height: usize,
    clear_color: u32,
    fps: u32,
}

impl EngineConfig {
    pub fn new(params: EngineConfigParams) -> Self {
        let default = EngineConfig::default();
        EngineConfig {
            window_title: params.window_title.unwrap_or(default.window_title),
            width: params.width.unwrap_gt_or(0, default.width),
            height: params.height.unwrap_gt_or(0, default.height),
            clear_color: params.clear_color.unwrap_or(default.clear_color),
            fps: params.fps.unwrap_gt_or(0, default.fps),
        }
    }

    pub fn window_title(&self) -> &String {
        &self.window_title
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        EngineConfig {
            window_title: "3D renderer".to_string(),
            width: 800,
            height: 600,
            clear_color: 0xFF000000,
            fps: 60,
        }
    }
}

type EngineUpdateFn<'a> = &'a mut dyn FnMut(&mut Engine);

pub struct EngineUpdate<'a> {
    previous_frame_time: Instant,
    target_frame_time: Duration,
    update_fn: EngineUpdateFn<'a>,
}

impl<'a> EngineUpdate<'a> {
    pub fn new(update_fn: EngineUpdateFn<'a>) -> Self {
        EngineUpdate {
            previous_frame_time: Instant::now(),
            target_frame_time: Duration::new(0, 1_000_000_000u32 / 60),
            update_fn,
        }
    }

    /*pub fn on_update(&mut self, update_fn: EngineUpdateFn<'a>) {
        self.update_fn = update_fn;
    }*/

    pub fn update(&mut self, engine: &mut Engine) {
        self.target_frame_time = Duration::new(0, 1_000_000_000u32 / engine.config.fps);
        let texture_creator = engine.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture(
                PixelFormatEnum::ARGB8888,
                sdl2::render::TextureAccess::Streaming,
                engine.config.width as u32,
                engine.config.height as u32,
            )
            .unwrap();

        let mut running = true;
        while running {
            self.previous_frame_time = Instant::now();

            running = engine.process_input();

            (self.update_fn)(engine);
            engine.render_buffer(&mut texture).unwrap();
            engine.clear_buffer();

            let now = Instant::now();
            let frame_time = now - self.previous_frame_time;
            println!(
                "Time this frame {}ms {} FPS",
                frame_time.as_millis(),
                1000u128 / frame_time.as_millis()
            );

            if frame_time.as_nanos() < self.target_frame_time.as_nanos() {
                ::std::thread::sleep(self.target_frame_time - frame_time);
            }
        }
    }
}

pub struct Engine<'a> {
    config: EngineConfig,
    canvas: Canvas<Window>,
    color_buffer: ColorBuffer,
    event_pump: EventPump,
    whatevs: Option<EngineUpdateFn<'a>>,
}

impl<'a> Engine<'a> {
    pub fn build(mut config: EngineConfig) -> Engine<'a> {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();

        config = EngineConfig {
            width: config.width,
            height: config.height,

            ..EngineConfig::default()
        };

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
            whatevs: None,
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
