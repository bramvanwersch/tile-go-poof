use sdl2;
use sdl2::event::Event;
use crate::game::{Game, GameState};


pub struct Input {
    events: sdl2::EventPump,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Input {
            events: sdl_context.event_pump().unwrap()
        }
    }

    pub fn handle_events(&mut self, game: &mut Game) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => game.game_state = GameState::QUIT,
                Event::MouseButtonDown { x, y,.. } => {
                    game.press_at_coord(x, y);
                }
                _ => { }
            }
        }
    }
}