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

    pub fn clear_color(&self) -> u32 {
        self.clear_color
    }

    pub fn fps(&self) -> u32 {
        self.fps
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

type EngineUpdateFn<'a> = &'a mut dyn FnMut(&mut EngineCore);

pub struct Engine<'a> {
    core: EngineCore,
    update: Option<EngineUpdateFn<'a>>,
    previous_frame_time: Instant,
    target_frame_time: Duration,
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

        let fps = config.fps;

        Engine {
            core: EngineCore {
                config,
                canvas,
                color_buffer,
                event_pump,
            },
            update: None,
            previous_frame_time: Instant::now(),
            target_frame_time: Duration::new(0, 1_000_000_000u32 / fps),
        }
    }

    pub fn config(&self) -> &EngineConfig {
        &self.core.config
    }

    pub fn on_update(&mut self, f: EngineUpdateFn<'a>) {
        self.update = Some(f);
        self.update();
    }

    pub fn user_update(&mut self) {
        self.update.as_mut().unwrap()(&mut self.core);
    }

    pub fn update(&mut self) {
        self.target_frame_time = Duration::new(0, 1_000_000_000u32 / self.core.config.fps);
        let texture_creator = self.core.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture(
                PixelFormatEnum::ARGB8888,
                sdl2::render::TextureAccess::Streaming,
                self.core.config.width as u32,
                self.core.config.height as u32,
            )
            .unwrap();

        let mut running = true;
        while running {
            self.previous_frame_time = Instant::now();

            running = self.core.process_input();

            self.user_update();
            self.core.render_buffer(&mut texture).unwrap();
            self.core.clear();

            let now = Instant::now();
            let frame_time = now - self.previous_frame_time;
            /*println!(
                "Time this frame {}ms {} FPS",
                frame_time.as_millis(),
                1000u128 / frame_time.as_millis()
            );*/

            if frame_time.as_nanos() < self.target_frame_time.as_nanos() {
                ::std::thread::sleep(self.target_frame_time - frame_time);
            }
        }
    }
}

pub struct EngineCore {
    config: EngineConfig,
    canvas: Canvas<Window>,
    color_buffer: ColorBuffer,
    event_pump: EventPump,
}

impl EngineCore {
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

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }
}

impl ClearAuto for EngineCore {
    fn clear(&mut self) {
        self.color_buffer.clear(self.config.clear_color);
    }
}

impl Drawable for EngineCore {
    fn draw_grid(&mut self, spacing: usize, color: Option<u32>) {
        self.color_buffer.draw_grid(spacing, color);
    }

    fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        self.color_buffer.draw_rect(x, y, width, height, color);
    }

    fn draw_point(&mut self, x: usize, y: usize, color: u32) {
        self.color_buffer.draw_point(x, y, color);
    }

    fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, color: u32) {
        self.color_buffer.draw_line(x0, y0, x1, y1, color);
    }
}
