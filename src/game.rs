use crate::constants::{COLS, ROWS, TILE_HEIGHT, TILE_WIDTH};

pub struct Game {
    // option for dynamic in future
    board: Vec<Vec<bool>>,
}

impl Game {
    pub fn new() -> Self {
        let mut board: Vec<Vec<bool>> = Vec::new();
        for _ in 0..ROWS {
            let mut row = Vec::new();
            for x in 0..COLS {
                if x % 9 == 0 {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
            board.push(row);
        }
        Game {
            board
        }
    }

    pub fn get_tile_state(&self, x: usize, y: usize) -> bool {
        *self.board.get(y).expect("Y out of range").get(x).expect("X out of range")
    }

    pub fn press_at_coord(&mut self, x: i32, y: i32) {
        let converted_x = (x as u32 / TILE_WIDTH) as usize;
        let converted_y = (y as u32 / TILE_HEIGHT) as usize;
        self.change_at_coord(converted_x, converted_y);
        if converted_y > 0 {
            self.change_at_coord(converted_x, converted_y - 1);
        }
        self.change_at_coord(converted_x, converted_y + 1);
        if converted_x > 0 {
            self.change_at_coord(converted_x - 1, converted_y);
        }
        self.change_at_coord(converted_x + 1, converted_y);
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