use std::collections::HashSet;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::game::{Game, GameState};


pub struct Input {
    events: sdl2::EventPump,
    prev_keys: HashSet<Keycode>
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Input {
            events: sdl_context.event_pump().unwrap(),
            prev_keys: HashSet::new()
        }
    }

    pub fn handle_events(&mut self, game: &mut Game) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => game.game_state = GameState::Quit,
                Event::MouseButtonDown { x, y,.. } => {
                    game.press_at_coord(x, y);
                },
                _ => { }
            }
        }
        // capture all new pressed keys
        let keys: HashSet<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        let new_keys = &keys - &self.prev_keys;
        for key in new_keys{
            match key {
                Keycode::Backspace => {
                    game.undo_move();
                },
                Keycode::R => {
                    game.game_state = GameState::NewGame;
                },
                Keycode::F =>{
                    game.game_state = GameState::LostGame;
                }
                _ => {}
            }
        }
        self.prev_keys = keys;
    }
}