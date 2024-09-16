use image::{ImageBuffer, Rgba};
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
    pub render_mode: Option<RenderMode>,
    pub backface_culling_enabled: Option<bool>,
}

impl Default for EngineConfigParams {
    fn default() -> Self {
        EngineConfigParams {
            window_title: None,
            width: None,
            height: None,
            clear_color: None,
            fps: None,
            render_mode: None,
            backface_culling_enabled: None,
        }
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        let width = 800;
        let height = 600;
        EngineConfig {
            window_title: "3D renderer".to_string(),
            width,
            height,
            clear_color: 0xFF000000,
            fps: 60,
            render_mode: RenderMode::Solid,
            backface_culling_enabled: true,
        }
    }
}

pub struct EngineConfig {
    window_title: String,
    width: usize,
    height: usize,
    clear_color: u32,
    fps: u32,
    render_mode: RenderMode,
    backface_culling_enabled: bool,
}

impl EngineConfig {
    pub fn new(params: EngineConfigParams) -> Self {
        let default = EngineConfig::default();

        let width = params.width.unwrap_gt_or(0, default.width);
        let height = params.height.unwrap_gt_or(0, default.height);

        EngineConfig {
            window_title: params.window_title.unwrap_or(default.window_title),
            width,
            height,
            clear_color: params.clear_color.unwrap_or(default.clear_color),
            fps: params.fps.unwrap_gt_or(0, default.fps),
            render_mode: params.render_mode.unwrap_or(default.render_mode),
            backface_culling_enabled: params
                .backface_culling_enabled
                .unwrap_or(default.backface_culling_enabled),
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

    pub fn aspect_ratio(&self) -> f64 {
        (self.height as f64) / (self.width as f64)
    }

    pub fn clear_color(&self) -> u32 {
        self.clear_color
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn render_mode(&self) -> &RenderMode {
        &self.render_mode
    }

    pub fn backface_culling_enabled(&self) -> bool {
        self.backface_culling_enabled
    }

    pub fn set_render_mode(&mut self, mode: RenderMode) {
        self.render_mode = mode;
    }

    pub fn set_backface_culling_enabled(&mut self, enabled: bool) {
        self.backface_culling_enabled = enabled;
    }
}

pub enum RenderMode {
    VerticesWireframe,
    Wireframe,
    Solid,
    SolidWireframe,
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
                config.fps = mode.refresh_rate as u32;
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

            match self.core.render_buffer(&mut texture) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("Failed to update texture: {}", e);
                }
            }

            let now = Instant::now();
            let frame_time = now - self.previous_frame_time;
            //println!("Frame: {}ms", frame_time.as_millis(),);

            if frame_time.as_nanos() < self.target_frame_time.as_nanos() {
                ::std::thread::sleep(self.target_frame_time - frame_time);
            }

            self.core.clear();
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
                // QUIT
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Received quit event, shutting down");
                    return false;
                }
                // KEYBOARD EVENTS
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let file_path: &str = "screenshot.jpg";
                    println!("Saving screenshot");
                    match self.save_screenshot(file_path) {
                        Ok(()) => (),
                        Err(_e) => eprint!("Failed to write screenshot"),
                    }
                    return true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num1),
                    ..
                } => {
                    println!("Wireframe and vertices");
                    self.config.set_render_mode(RenderMode::VerticesWireframe);
                    return true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num2),
                    ..
                } => {
                    println!("Wireframe");
                    self.config.set_render_mode(RenderMode::Wireframe);
                    return true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num3),
                    ..
                } => {
                    println!("Solid");
                    self.config.set_render_mode(RenderMode::Solid);
                    return true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Num4),
                    ..
                } => {
                    println!("Solid Wireframe");
                    self.config.set_render_mode(RenderMode::SolidWireframe);
                    return true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    println!("Enable back-face culling");
                    self.config.set_backface_culling_enabled(true);
                    return true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    println!("Disable back-face culling");
                    self.config.set_backface_culling_enabled(false);
                    return true;
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
        /*texture.update(None, self.color_buffer.pixels(), self.config.width * 4)?;
        self.canvas.copy(texture, None, None).unwrap();*/

        texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                let pixels = self.color_buffer.pixels(); // Get the pixel data from color_buffer
                buffer.copy_from_slice(pixels); // Copy pixel data into the locked texture buffer
            })
            .map_err(UpdateTextureError::SdlError)?;

        self.canvas.copy(texture, None, None).unwrap();

        Ok(())
    }

    fn save_screenshot(&mut self, file_path: &str) -> Result<(), String> {
        let (width, height) = self.canvas.output_size().map_err(|e| e.to_string())?;

        // Read pixels from the canvas
        let pixel_data = self
            .canvas
            .read_pixels(None, PixelFormatEnum::RGB888)
            .map_err(|e| e.to_string())?;

        // Convert the pixel data to an ImageBuffer
        let img =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, pixel_data)
                .ok_or("Failed to create image buffer")?;

        // Save the image as PNG
        img.save(file_path).map_err(|e| e.to_string())?;

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
