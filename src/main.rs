use raylib::{color::Color, consts::KeyboardKey, prelude::RaylibDraw};
use universe::Universe;

mod particle;
mod universe;

const WIDTH: i32 = 1600;
const HEIGHT: i32 = 900;
const STEPS_PER_FRAME: usize = 20;
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .build();
    let mut universe = Universe::new(4, 400, WIDTH as f32, HEIGHT as f32);
    universe.re_seed(-0.02, 0.06, 0.0, 20.0, 20.0, 70.0, 0.05, false);
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
            Some(KeyboardKey::KEY_UP) => {}
            _ => {}
        }

        let mut d = rl.begin_drawing(&thread);

        for i in 0..STEPS_PER_FRAME {
            universe.step();
        }
        universe.draw(&mut d, 1.0);
        d.clear_background(Color::BLACK);
    }
}
