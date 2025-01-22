use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_ITER: u32 = 255;

fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = c_re;
    let mut z_im = c_im;
    for i in 0..max_iter {
        if z_re * z_re + z_im * z_im > 4.0 {
            return i;
        }
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re * z_im + c_im;
        z_re = new_re;
        z_im = new_im;
    }
    max_iter
}

fn draw_mandelbrot(canvas: &mut Canvas<Window>) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let c_re = (x as f64 - WIDTH as f64 / 2.0) * 4.0 / WIDTH as f64;
            let c_im = (y as f64 - HEIGHT as f64 / 2.0) * 4.0 / HEIGHT as f64;
            let iter = mandelbrot(c_re, c_im, MAX_ITER);
            let color = (iter as u8, iter as u8, iter as u8);
            canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
            let _ = canvas.draw_point((x as i32, y as i32));
        }
    }
}

fn draw() {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot Set", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    draw_mandelbrot(&mut canvas);
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
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
    }
}
