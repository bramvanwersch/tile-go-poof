extern crate sdl2;

use crate::game::{Game, GameState};
use crate::input::Input;
use crate::windows::GameDisplay;

mod windows;
mod input;
mod game;
mod constants;
mod move_history;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("./data/DisposableDroidBB.ttf", 64).expect("Failed to read font");

    let mut display: GameDisplay = GameDisplay::new(&sdl_context, font);
    let mut input = Input::new(&sdl_context);

    let mut game = Game::new();
    game.next_level();
    loop {
        input.handle_events(&mut game);
        match game.game_state {
            GameState::LevelFinished => {
                game.next_level();
            },
            GameState::LostGame =>{
                game.record_highscore();
            },
            GameState::NewGame =>{
                game = Game::new();
                game.next_level();
            },
            GameState::Quit =>{
                return;
            },
            _ => {}
        }
        display.refresh(&mut game);
    }
}
