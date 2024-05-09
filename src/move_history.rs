use crate::constants::{COLS, ROWS};

pub struct MoveHistory{
    buffer: Vec<String>
}

impl MoveHistory{
    pub fn new() -> Self{
        MoveHistory{
            buffer: Vec::new()
        }
    }

    pub fn save_board_state(&mut self, board: &Vec<Vec<bool>>){
        let mut sequence: String = String::new();
        for y in 0..ROWS {
            for x in 0..COLS {
                if board[y][x]{
                    sequence += "1";
                }
                else {
                    sequence += "0";
                }
            }
        }
        self.buffer.push(sequence);
    }

    pub fn empty(&mut self){
        self.buffer = Vec::new();
    }

    pub fn get_board_state(&self, offset: usize) -> &String{
        self.buffer.get(self.buffer.len() - 1 - offset).expect("History value out of range")
    }

    pub fn pop_board_state(&mut self) -> String{
        self.buffer.pop().expect("Buffer is empty")
    }

    pub fn len(&self) -> usize{
        self.buffer.len()
    }
}