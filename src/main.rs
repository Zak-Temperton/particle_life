use raylib::{color::Color, prelude::RaylibDraw};
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
    universe.re_seed(-0.2, 0.5, 0.0, 20.0, 10.0, 150.0, 0.5, false);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        for _ in 0..STEPS_PER_FRAME {
            universe.step();
        }
        if d.is_key_pressed(raylib::consts::KeyboardKey::KEY_R) {
            universe.re_seed(-0.2, 0.5, 0.0, 20.0, 10.0, 150.0, 0.5, false);
            println!("{}", &universe);
        }
        universe.draw(&mut d);
        d.clear_background(Color::BLACK);
    }
}
