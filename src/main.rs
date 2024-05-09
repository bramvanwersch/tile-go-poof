extern crate sdl2;

use crate::game::Game;
use crate::input::Input;
use crate::window::Display;

mod window;
mod input;
mod game;
mod constants;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    let mut game = Game::new();

    while let Ok(_keypad) = input.handle_events(&mut game){
        display.refresh(&game);
    }
}
