extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::FPoint;

use std::time::Duration;

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn to_screen(&self, w: u32, h: u32) -> (i32, i32) {
        (
            (((self.x / (self.z + 1.0)) + 1.0) / 2.0 * w as f64).round() as i32,
            ((1.0 - ((self.y / (self.z + 1.0)) + 1.0) / 2.0) * h as f64).round() as i32,
        )
    }

    fn rotate_x(&mut self, angle: f64) -> &mut Self {
        let y = self.y;
        let z = self.z;
        self.y = angle.cos() * y - angle.sin() * z;
        self.z = angle.sin() * y + angle.cos() * z;
        self
    }

    fn rotate_y(&mut self, angle: f64) -> &mut Self {
        let x = self.x;
        let z = self.z;
        self.x = angle.cos() * x + angle.sin() * z;
        self.z = -angle.sin() * x + angle.cos() * z;
        self
    }

    fn rotate_z(&mut self, angle: f64) -> &mut Self {
        let x = self.x;
        let y = self.y;
        self.x = angle.cos() * x - angle.sin() * y;
        self.y = angle.sin() * x + angle.cos() * y;
        self
    }
}

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let bg_color = Color::RGB(0, 0, 0);
    let fg_color = Color::RGB(40, 255, 40);
    let win_w = 800;
    let win_h = 800;
    let move_speed = 1.0 / 60.0;
    let zoom_enabled = true;
    let rotation_angle = 0.01;

    let window = video_subsystem
        .window("rust-sdl3 demo", win_w, win_h)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();
    let original_points = [
        Point {
            x: 0.25,
            y: 0.25,
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
            y: -0.25,
            z: -0.25,
        },
        Point {
            x: 0.25,
            y: 0.25,
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
        Point {
            x: 0.25,
            y: -0.25,
            z: 0.25,
        },
    ];

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut zoom_offset = 0.0;
    let mut zoom_out = true;
    let mut current_rotation_x = 0.0;
    let mut current_rotation_y = 0.0;
    let mut fpoints = Vec::new();

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

        if zoom_enabled {
            if zoom_out {
                zoom_offset += move_speed;
                if zoom_offset >= 2.25 {
                    zoom_out = false;
                }
            } else {
                zoom_offset -= move_speed;
                if zoom_offset <= 0.0 {
                    zoom_out = true;
                }
            }
        }

        current_rotation_x += rotation_angle;
        current_rotation_y += rotation_angle;

        fpoints.clear();
        for orig_point in original_points.iter() {
            let mut point = Point {
                x: orig_point.x,
                y: orig_point.y,
                z: orig_point.z,
            };

            point.rotate_x(current_rotation_x);
            point.rotate_y(current_rotation_y);
            point.z += zoom_offset;

            let point_to_screen = point.to_screen(win_w, win_h);
            fpoints.push(FPoint::new(
                point_to_screen.0 as f32,
                point_to_screen.1 as f32,
            ));
        }

        // canvas.draw_points(fpoints.as_slice()).unwrap();
        canvas.draw_line(fpoints[0], fpoints[1]).unwrap();
        canvas.draw_line(fpoints[1], fpoints[2]).unwrap();
        canvas.draw_line(fpoints[2], fpoints[3]).unwrap();
        canvas.draw_line(fpoints[3], fpoints[0]).unwrap();

        canvas.draw_line(fpoints[4], fpoints[5]).unwrap();
        canvas.draw_line(fpoints[5], fpoints[6]).unwrap();
        canvas.draw_line(fpoints[6], fpoints[7]).unwrap();
        canvas.draw_line(fpoints[7], fpoints[4]).unwrap();

        canvas.draw_line(fpoints[0], fpoints[4]).unwrap();
        canvas.draw_line(fpoints[1], fpoints[5]).unwrap();
        canvas.draw_line(fpoints[2], fpoints[6]).unwrap();
        canvas.draw_line(fpoints[3], fpoints[7]).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
