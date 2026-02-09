extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;

use std::time::Duration;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn project(&mut self, w: u32, h: u32) -> &Self {
        self.x = self.x / self.z * w as f64;
        self.y = self.y / self.z * h as f64;
        self
    }

    fn to_screen(&self, w: u32, h: u32) -> (i32, i32) {
        (
            ((self.x + 1.0) / 2.0 * w as f64).round() as i32,
            ((self.y + 1.0) / 2.0 * h as f64).round() as i32,
        )
    }
}

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let bg_color = Color::RGB(0, 0, 0);
    let fg_color = Color::RGB(40, 255, 40);
    let win_w = 800;
    let win_h = 600;
    let move_speed = 5;
    let point_size = 10;

    let window = video_subsystem
        .window("rust-sdl3 demo", win_w, win_h)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    let mut point = Point {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    let mut i: f64 = 0.0;
    let mut j: f64 = 0.0;
    let mut point_to_screen = point.to_screen(win_w, win_h);
    let mut rect = Rect::new(point_to_screen.0, point_to_screen.1, point_size, point_size);

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
        // The rest of the game loop goes here...
        // i = (i + move_speed) % win_w as i32;
        // j = (j + move_speed) % win_h as i32;

        canvas.set_draw_color(bg_color);
        canvas.clear();
        canvas.set_draw_color(fg_color);
        point_to_screen = point.to_screen(win_w, win_h);
        rect.set_x(point_to_screen.0);
        rect.set_y(point_to_screen.1);

        canvas.fill_rect(rect).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
