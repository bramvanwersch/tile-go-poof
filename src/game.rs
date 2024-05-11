use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::path::Path;
use rand::Rng;
use crate::constants::{COLS, MAX_TIME, PLAY_HEIGHT, PLAY_WIDTH, ROWS, START_DIFFICULTY, TILE_HEIGHT, TILE_WIDTH};
use crate::move_history::MoveHistory;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Playing,
    LevelFinished,
    LostGame,
    Quit,
    NewGame,
    HighScore
}

pub struct Game {
    // option for dynamic in future
    board: Vec<Vec<bool>>,
    history: MoveHistory,
    pub score: f32,
    level_start_time: std::time::Instant,
    pub level: usize,
    difficulty: usize,
    pub game_state: GameState,
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
            history: MoveHistory::new(),
            score: 0.0,
            level_start_time: std::time::Instant::now(),
            level: 0,
            difficulty: START_DIFFICULTY,
            game_state: GameState::Playing,
        }
    }

    pub fn next_level(&mut self) {
        self.history.empty();
        if self.difficulty != START_DIFFICULTY{
            let difficulty_factor = 1.0 + self.difficulty as f32 / 50.0;
            let time_factor =  1.0 + self.time_left() as f32 / 250.0;
            self.score += (self.level as f32 * difficulty_factor * time_factor).max(1.0);
        }
        self.difficulty += 1;
        self.level += 1;
        self.level_start_time = std::time::Instant::now();
        let mut level_create_history = MoveHistory::new();
        let mut covered_states: HashSet<String> = HashSet::new();
        let mut counter = 0;
        while counter < self.difficulty{
            let x = rand::thread_rng().gen_range(0..COLS);
            let y = rand::thread_rng().gen_range(0..ROWS);
            self.press_at_tile(x, y);
            let state = level_create_history.get_state_string(&self.board);
            if !covered_states.contains(&state){
                covered_states.insert(state);
                counter += 1;
            }
        }
        self.game_state = GameState::Playing;
    }

    pub fn record_highscore(&mut self){
        let hs_path = "./data/highscores.txt";
        if !Path::new(hs_path).exists(){
            File::create(hs_path).expect("Failed to create the highscore file");
        }
        let mut contents = fs::read_to_string(hs_path).expect("Failed to read the highscore file");
        let mut scores: Vec<f32> = Vec::new();
        scores.push(self.score);
        for line in contents.lines(){
            scores.push(line.parse::<f32>().unwrap())
        }
        let mut new_contents = String::new();
        scores.sort_by(|a, b| b.partial_cmp(a).unwrap());
        for (index, num) in scores.iter().enumerate(){
            new_contents.push_str(format!("{}", num).as_str());
            new_contents.push_str("\n");
            if index == 9{
                break;
            }
        }
        fs::write(hs_path, new_contents).expect("Failed to write highscores");
        self.game_state = GameState::HighScore;
    }

    pub fn set_state(&mut self){
        if self.game_state == GameState::Playing{
            if self.time_left() < 0{
                self.game_state = GameState::LostGame;
            }
            for y in 0..ROWS {
                for x in 0..COLS {
                    if self.board[y][x] {
                        return;
                    }
                }
            }
            self.game_state = GameState::LevelFinished;
        }
    }

    pub fn undo_move(&mut self) {
        if self.history.len() == 0{
            return;
        }
        let last_move = self.history.pop_board_state();
        for (index, char) in last_move.chars().enumerate() {
            self.board[index / ROWS][index % COLS] = char == '1';
        }
    }

    pub fn get_tile_state(&self, x: usize, y: usize) -> bool {
        *self.board.get(y).expect("Y out of range").get(x).expect("X out of range")
    }

    pub fn press_at_coord(&mut self, x: i32, y: i32) {
        if x >= PLAY_WIDTH as i32 || y >= PLAY_HEIGHT as i32{
            return;
        }
        self.history.save_board_state(&self.board);
        let converted_x = (x as u32 / TILE_WIDTH) as usize;
        let converted_y = (y as u32 / TILE_HEIGHT) as usize;
        self.press_at_tile(converted_x, converted_y);
    }

    pub fn time_left(&self) -> i32{
        return MAX_TIME as i32 - self.level_start_time.elapsed().as_secs() as i32;
    }

    fn press_at_tile(&mut self, x: usize, y: usize) {
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