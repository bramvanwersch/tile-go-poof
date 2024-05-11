extern crate sdl2;

use std::fs;
use std::io::Read;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use crate::constants::{ACTIVE_COLOR, BORDER_COLOR, COLS, INACTIVE_COLOR, ROWS, TILE_HEIGHT, TILE_WIDTH, HEIGHT, WIDTH, PLAY_HEIGHT, BOTTOM_BAR, TEXT_COLOR};
use crate::game::{Game, GameState};

pub struct GameDisplay<'a> {
    canvas: Canvas<Window>,
    font: Font<'a, 'a>,
    texture_creator: TextureCreator<WindowContext>
}

impl<'a> GameDisplay<'a> {
    pub fn new(sdl_context: &sdl2::Sdl, mut font: Font<'a, 'a>) -> Self {
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("tile-go-poof", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let texture_creator = canvas.texture_creator();
        GameDisplay {
            canvas,
            font,
            texture_creator
        }
    }

    pub fn refresh(&mut self, game: &mut Game){
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
        game.set_state();
        if game.game_state == GameState::Playing{
            self.draw_game(game);
        }
        else if game.game_state== GameState::HighScore{
            self.draw_highscore(game);
        }
        self.canvas.present();
    }

    fn draw_game(&mut self, game: &Game){
        let x_tile_size = TILE_WIDTH;
        let y_tile_size = TILE_HEIGHT;
        for y in 0..COLS{
            for x in 0..ROWS{
                let color = match game.get_tile_state(x, y){
                    true => ACTIVE_COLOR,
                    false => INACTIVE_COLOR
                };
                self.canvas.set_draw_color(color);
                let rect = Rect::new((x as u32 * x_tile_size) as i32, (y as u32 * y_tile_size) as i32, x_tile_size, y_tile_size);
                let _ = self.canvas.fill_rect(rect);
                self.canvas.set_draw_color(BORDER_COLOR);
                let _ = self.canvas.draw_rect(rect);
            }
        }
        self.draw_bottom(game);
    }

    fn draw_bottom(&mut self, game: &Game){
        let text = format!("Level: {} Time left: {} Score: {:.2}", game.level, game.time_left(), game.score);
        self.draw_text(text.as_str(), Rect::new(10, PLAY_HEIGHT as i32, text.len() as u32 * 10, BOTTOM_BAR));
    }

    fn draw_highscore(&mut self, game: &Game){
        let hs_path = "./data/highscores.txt";
        let mut contents = fs::read_to_string(hs_path).expect("Failed to read the highscore file");
        let mut scores: Vec<f32> = Vec::new();
        for line in contents.lines(){
            scores.push(line.parse::<f32>().unwrap())
        }
        let mut text = format!("You scored: {:.2}\n", game.score);
        for (index, score) in scores.iter().enumerate(){
            text.push_str(format!("{}. {:.2}\n", index + 1, score).as_str());
            if index == 9{
                break;
            }
        }
        text.push_str("Press 'r' to play again");
        self.draw_text_nl(text.as_str(), Rect::new(5, 5, HEIGHT, WIDTH));
    }

    fn draw_text(&mut self, text: &str, rect: Rect){
        let surface = self.font
            .render(text).blended(TEXT_COLOR)
            .unwrap();
        let texture = self.texture_creator
            .create_texture_from_surface(&surface).unwrap();
        self.canvas.copy(&texture, None, Some(rect)).unwrap();
        self.canvas.present();
    }

    fn draw_text_nl(&mut self, text: &str, rect: Rect){
        let surface = self.font
            .render(text).blended_wrapped(TEXT_COLOR, WIDTH)
            .unwrap();
        let texture = self.texture_creator
            .create_texture_from_surface(&surface).unwrap();
        self.canvas.copy(&texture, None, Some(rect)).unwrap();
        self.canvas.present();
    }
}