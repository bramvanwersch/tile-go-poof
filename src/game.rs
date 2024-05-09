use rand::Rng;
use crate::constants::{COLS, ROWS, TILE_HEIGHT, TILE_WIDTH};

#[derive(PartialEq, Eq)]
pub enum GameState{
    PLAYING,
    QUIT
}

pub struct Game {
    // option for dynamic in future
    board: Vec<Vec<bool>>,
    pub game_state: GameState
}

impl Game {
    pub fn new() -> Self {
        let mut board: Vec<Vec<bool>> = Vec::new();
        for _ in 0..ROWS {
            let mut row = Vec::new();
            for _ in 0..COLS {
                row.push(false);
            }
            board.push(row);
        }
        Game {
            board,
            game_state: GameState::PLAYING
        }
    }

    pub fn next_level(&mut self, difficulty: usize){
        for _ in 0..difficulty{
            let x = rand::thread_rng().gen_range(0..COLS);
            let y = rand::thread_rng().gen_range(0..ROWS);
            self.press_at_tile(x, y);
        }
    }

    pub fn level_finished(&self) -> bool{
        for y in 0..ROWS {
            for x in 0..COLS {
                if self.board[y][x] {
                    return false
                }
            }
        }
        return true
    }

    pub fn get_tile_state(&self, x: usize, y: usize) -> bool {
        *self.board.get(y).expect("Y out of range").get(x).expect("X out of range")
    }

    pub fn press_at_coord(&mut self, x: i32, y: i32) {
        let converted_x = (x as u32 / TILE_WIDTH) as usize;
        let converted_y = (y as u32 / TILE_HEIGHT) as usize;
        self.press_at_tile(converted_x, converted_y);
    }

    fn press_at_tile(&mut self, x: usize, y:usize){
        self.change_at_coord(x, y);
        if y > 0 {
            self.change_at_coord(x, y - 1);
        }
        self.change_at_coord(x, y + 1);
        if x > 0 {
            self.change_at_coord(x - 1, y);
        }
        self.change_at_coord(x + 1, y);
    }

    fn change_at_coord(&mut self, x: usize, y: usize) {
        let row = match self.board.get(y) {
            Some(res) => res,
            None => return
        };
        match row.get(x) {
            Some(res) => self.board[y][x] = !res,
            None => return
        };
    }
}