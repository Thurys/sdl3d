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
    fn to_screen(&self, w: u32, h: u32) -> (i32, i32) {
        (
            (((self.x / self.z) + 1.0) / 2.0 * w as f64).round() as i32,
            ((1.0 - ((self.y / self.z) + 1.0) / 2.0) * h as f64).round() as i32,
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
    let move_speed = 1.0 / 60.0;
    let point_size = 2;

    let window = video_subsystem
        .window("rust-sdl3 demo", win_w, win_h)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    let mut points = [
        Point {
            x: 0.25,
            y: 0.25,
            z: -0.25,
        },
        Point {
            x: 0.25,
            y: -0.25,
            z: -0.25,
        },
        Point {
            x: -0.25,
            y: 0.25,
            z: -0.25,
        },
        Point {
            x: -0.25,
            y: -0.25,
            z: -0.25,
        },
        Point {
            x: 0.25,
            y: 0.25,
            z: 0.25,
        },
        Point {
            x: 0.25,
            y: -0.25,
            z: 0.25,
        },
        Point {
            x: -0.25,
            y: 0.25,
            z: 0.25,
        },
        Point {
            x: -0.25,
            y: -0.25,
            z: 0.25,
        },
    ];

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut zoom_out = true;

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
        canvas.set_draw_color(bg_color);
        canvas.clear();
        canvas.set_draw_color(fg_color);

        let mut rects = Vec::new();

        if points.iter().all(|r| r.z.abs() >= 2.5) {
            zoom_out = !zoom_out;
        } else if points.iter().all(|r| r.z.abs() <= 0.25) {
            zoom_out = !zoom_out;
        }

        points.iter_mut().for_each(|point| {
            if zoom_out {
                point.z += move_speed;
            } else {
                point.z -= move_speed;
            }
            let point_to_screen = point.to_screen(win_w, win_h);
            rects.push(Rect::new(
                point_to_screen.0,
                point_to_screen.1,
                point_size,
                point_size,
            ));
        });
        rects.iter().for_each(|rect| {
            canvas.fill_rect(*rect).unwrap();
        });

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
