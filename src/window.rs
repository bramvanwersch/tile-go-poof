extern crate sdl2;

use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::constants::{ACTIVE_COLOR, BORDER_COLOR, COLS, HEIGHT, INACTIVE_COLOR, ROWS, TILE_HEIGHT, TILE_WIDTH, WIDTH};
use crate::game::Game;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("tile-go-poof", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(pixels::Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();
        Display {
            canvas,
        }
    }

    pub fn refresh(&mut self, game_state: &Game){
        let x_tile_size = TILE_WIDTH;
        let y_tile_size = TILE_HEIGHT;
        for y in 0..COLS{
            for x in 0..ROWS{
                let color = match game_state.get_tile_state(x, y){
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
        self.canvas.present();
    }
}
