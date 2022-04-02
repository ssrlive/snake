#![windows_subsystem = "windows"]

extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

macro_rules! o_O {
    ( $($x:expr; [ $( $y:expr ),* ] );* ) =>
    {
        &[ $($( $x + $y ),*),* ]
    }
}

fn _main() {
    let a: &[i32] = o_O!(10; [1, 2, 3]; 20; [4, 5, 6]);

    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    let x = 5;
    let raw = &x as *const i32;

    let points_at = unsafe { *raw };

    println!("raw points at {}", points_at);
}

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
