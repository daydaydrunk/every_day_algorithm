use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::ttf::{self, Font};
use sdl2::video::Window;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const SAMPLE_RATE: i32 = 44100;
const SAMPLE_SIZE: usize = 1024;
const PIXEL_SIZE: i32 = 4; // 像素块大小
const BLUR_AMOUNT: f32 = 0.8; // 像素模糊程度

struct AudioState {
    buffer: Vec<f32>,
    index: usize,
}

struct AudioInputCallback {
    state: std::sync::Arc<std::sync::Mutex<AudioState>>,
}

impl AudioCallback for AudioInputCallback {
    type Channel = f32;

    fn callback(&mut self, input: &mut [f32]) {
        let mut state = self.state.lock().unwrap();
        for (i, &sample) in input.iter().enumerate() {
            let idx = state.index;
            state.buffer[idx] = sample;
            state.index = (state.index + 1) % SAMPLE_SIZE;
        }
    }
}

fn draw_pixel_grid(canvas: &mut Canvas<Window>, x: i32, y: i32, intensity: u8) {
    let pixel_rect = Rect::new(
        (x / PIXEL_SIZE) * PIXEL_SIZE,
        (y / PIXEL_SIZE) * PIXEL_SIZE,
        PIXEL_SIZE as u32,
        PIXEL_SIZE as u32,
    );

    // 主像素
    canvas.set_draw_color(Color::RGBA(0, intensity, 0, 255));
    canvas.fill_rect(pixel_rect).unwrap();

    // 像素扩散效果 - 手动计算扩展区域
    canvas.set_blend_mode(sdl2::render::BlendMode::Add);
    canvas.set_draw_color(Color::RGBA(0, (intensity as f32 * 0.3) as u8, 0, 100));

    let glow_rect = Rect::new(
        pixel_rect.x() - 2,
        pixel_rect.y() - 2,
        pixel_rect.width() + 4,
        pixel_rect.height() + 4,
    );
    canvas.fill_rect(glow_rect).unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::None);
}

fn apply_crt_distortion(x: f32, y: f32) -> (f32, f32) {
    let nx = x / WINDOW_WIDTH as f32 * 2.0 - 1.0;
    let ny = y / WINDOW_HEIGHT as f32 * 2.0 - 1.0;

    // 添加桶形失真
    let dist = nx * nx + ny * ny;
    let barrel = 0.1; // 失真程度

    let dx = nx * (1.0 + barrel * dist);
    let dy = ny * (1.0 + barrel * dist);

    (
        (dx + 1.0) * WINDOW_WIDTH as f32 / 2.0,
        (dy + 1.0) * WINDOW_HEIGHT as f32 / 2.0,
    )
}

fn draw_crt_effect(canvas: &mut Canvas<Window>) {
    // 增强的扫描线效果
    for y in (0..WINDOW_HEIGHT).step_by(2) {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 150));
        canvas
            .fill_rect(Rect::new(0, y as i32, WINDOW_WIDTH, 1))
            .unwrap();
    }

    // 添加微弱的垂直扫描线
    for x in (0..WINDOW_WIDTH).step_by(4) {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 50));
        canvas
            .fill_rect(Rect::new(x as i32, 0, 1, WINDOW_HEIGHT))
            .unwrap();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("CRT Oscilloscope", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let desired_spec = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(1),
        samples: Some(SAMPLE_SIZE as u16),
    };

    let audio_state = std::sync::Arc::new(std::sync::Mutex::new(AudioState {
        buffer: vec![0.0; SAMPLE_SIZE],
        index: 0,
    }));

    let audio_callback = AudioInputCallback {
        state: audio_state.clone(),
    };

    let device = audio_subsystem.open_capture(None, &desired_spec, |spec| AudioInputCallback {
        state: audio_state.clone(),
    })?;

    device.resume();

    let mut event_pump = sdl_context.event_pump()?;

    // Load a font
    let font_path = "AmericanTypewriter.ttf";
    let font = ttf_context.load_font(font_path, 128)?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Clear screen with dark background
        canvas.set_draw_color(Color::RGB(5, 5, 5));
        canvas.clear();

        // Draw waveform
        let state = audio_state.lock().unwrap();
        let buffer = &state.buffer;

        // 像素化波形渲染
        for i in 0..buffer.len() - 1 {
            let x = (i as f32 / buffer.len() as f32 * WINDOW_WIDTH as f32) as i32;
            let y = (WINDOW_HEIGHT as f32 / 2.0 + buffer[i] * WINDOW_HEIGHT as f32 / 3.0) as i32;

            // 应用CRT失真
            let (distorted_x, distorted_y) = apply_crt_distortion(x as f32, y as f32);

            // 绘制像素化点
            let intensity = (255.0 * (1.0 - (buffer[i].abs() * 0.5))) as u8;
            draw_pixel_grid(
                &mut canvas,
                distorted_x as i32,
                distorted_y as i32,
                intensity,
            );
        }

        // Apply CRT effect
        draw_crt_effect(&mut canvas);

        // 增强的余晕效果
        canvas.set_blend_mode(sdl2::render::BlendMode::Add);
        canvas.set_draw_color(Color::RGBA(0, 30, 0, 3));
        canvas.fill_rect(None)?;
        canvas.set_blend_mode(sdl2::render::BlendMode::None);

        // Create a texture for the text
        let surface = font
            .render("OCS")
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // Get the texture query to determine the size of the text
        let TextureQuery { width, height, .. } = texture.query();

        // Define the position for the text (near the bottom of the window)
        let target = Rect::new(
            (WINDOW_WIDTH / 2 - width / 2) as i32,
            (WINDOW_HEIGHT - height - 20) as i32,
            width,
            height,
        );

        // Render the text
        canvas.copy(&texture, None, Some(target))?;

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
