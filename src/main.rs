use std::time::{Duration, Instant};

use camera::Camera;
use raylib::{
    color::Color,
    consts::{KeyboardKey, MouseButton},
    prelude::RaylibDraw,
};
use universe::Universe;

mod camera;
mod particle;
mod universe;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 900;

fn min(v1: f32, v2: f32) -> f32 {
    v1.min(v2)
}

fn max(v1: f32, v2: f32) -> f32 {
    v1.max(v2)
}

fn main() {
    let mut steps_per_frame = 20;
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .vsync()
        .build();
    let mut universe = Universe::new(4, 400, WIDTH as f32, HEIGHT as f32);
    universe.re_seed(-0.02, 0.06, 0.0, 20.0, 20.0, 70.0, 0.05, false);
    let mut cam = Camera::new();
    println! {"
=========================================================

                Welcome to Particle Life
This is a particle-based game of life simulation based
on random attraction and repulsion between all particle
classes.
=========================================================
    Controls:
            'B' - Randomize (Balanced)
            'C' - Randomize (Chaos)
            'D' - Randomize (Diversity)
            'F' - Randomize (Frictionless)
            'G' - Randomize (Gliders)
            'H' - Randomize (Homogeneity)
            'L' - Randomize (Large Clusters)
            'M' - Randomize (Medium Clusters)
            'Q' - Randomize (Quiescence)
            'S' - Randomize (Small Clusters)
            'W' - Toggle Wrap-Around
        Enter - Keep rules, but re-seed particles
        Space - Hold for slow-motion
            Tab - Print current parameters to console
    Left Click - Click a particle to follow it
    Right Click - Click anywhere to unfollow particle
Scroll Wheel - Zoom in/out
"};

    while !rl.window_should_close() {
        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_B) => {
                universe.set_population(9, 400);
                universe.re_seed(-0.02, 0.06, 0.0, 20.0, 20.0, 70.0, 0.05, false);
            }
            Some(KeyboardKey::KEY_C) => {
                universe.set_population(6, 400);
                universe.re_seed(0.02, 0.04, 0.0, 30.0, 30.0, 100.0, 0.01, false);
            }
            Some(KeyboardKey::KEY_D) => {
                universe.set_population(12, 400);
                universe.re_seed(-0.01, 0.04, 0.0, 20.0, 10.0, 60.0, 0.05, true);
            }
            Some(KeyboardKey::KEY_F) => {
                universe.set_population(6, 300);
                universe.re_seed(0.01, 0.05, 10.0, 10.0, 10.0, 60.0, 0.0, true);
            }
            Some(KeyboardKey::KEY_G) => {
                universe.set_population(6, 400);
                universe.re_seed(0.0, 0.06, 0.0, 20.0, 10.0, 50.0, 0.1, true);
            }
            Some(KeyboardKey::KEY_H) => {
                universe.set_population(4, 400);
                universe.re_seed(0.0, 0.04, 10.0, 10.0, 10.0, 80.0, 0.05, true);
            }
            Some(KeyboardKey::KEY_L) => {
                universe.set_population(6, 400);
                universe.re_seed(0.025, 0.02, 0.0, 30.0, 30.0, 100.0, 0.2, false);
            }
            Some(KeyboardKey::KEY_M) => {
                universe.set_population(6, 400);
                universe.re_seed(0.02, 0.05, 0.0, 20.0, 20.0, 50.0, 0.05, false);
            }
            Some(KeyboardKey::KEY_Q) => {
                universe.set_population(6, 300);
                universe.re_seed(-0.02, 0.1, 10.0, 20.0, 20.0, 60.0, 0.2, false);
            }
            Some(KeyboardKey::KEY_S) => {
                universe.set_population(6, 300);
                universe.re_seed(-0.005, 0.01, 10.0, 10.0, 20.0, 50.0, 0.01, false);
            }
            Some(KeyboardKey::KEY_W) => universe.toggle_wrap(),
            Some(KeyboardKey::KEY_ENTER) => universe.set_random_particles(),
            Some(KeyboardKey::KEY_TAB) => println!("{}", &universe),
            Some(KeyboardKey::KEY_SPACE) => {
                if steps_per_frame == 1 {
                    steps_per_frame = 20
                } else {
                    steps_per_frame = 1
                }
            }
            // Some(KeyboardKey::KEY_UP) => {
            //     *cam.zoom_dest_mut() *= 1.1;
            //     dbg!(cam.zoom_dest());
            // }
            _ => {
                let mouse_pos = rl.get_mouse_position();
                if rl.get_mouse_wheel_move() != 0.0 {
                    *cam.zoom_dest_mut() *= 1.1_f32.powf(rl.get_mouse_wheel_move());
                    *cam.zoom_dest_mut() = max(min(cam.zoom_dest(), 10.0), 1.0);
                    let cur_time = Instant::now();
                    if cur_time.duration_since(cam.last_scroll_time()) > Duration::from_millis(300)
                    {
                        universe.to_centre(mouse_pos.x as usize, mouse_pos.y as usize, &mut cam);
                    }
                } else if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                    *cam.track_index_mut() =
                        universe.get_index(mouse_pos.x as usize, mouse_pos.y as usize);
                } else if rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
                    *cam.x_dest_mut() = (WIDTH / 2) as f32;
                    *cam.y_dest_mut() = (HEIGHT / 2) as f32;
                    *cam.track_index_mut() = None;
                }
            }
        }
        cam.apply_zoom(&mut universe);

        let mut d = rl.begin_drawing(&thread);

        for _ in 0..steps_per_frame {
            universe.step();
        }
        universe.draw(&mut d, 1.0);
        d.clear_background(Color::BLACK);
    }
}
