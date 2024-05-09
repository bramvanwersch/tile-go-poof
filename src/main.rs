extern crate sdl2;

use crate::game::{Game, GameState};
use crate::input::Input;
use crate::window::Display;

mod window;
mod input;
mod game;
mod constants;
mod move_history;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);
    let mut input = Input::new(&sdl_context);

    let mut game = Game::new();
    let mut difficulty = 1;
    game.next_level(difficulty);
    while game.game_state == GameState::PLAYING{
        input.handle_events(&mut game);
        if game.game_state == GameState::QUIT {
            break;
        }
        if game.level_finished(){
            difficulty += 1;
            game.next_level(difficulty)
        }
        display.refresh(&game);
    }
}
