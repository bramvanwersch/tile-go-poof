use sdl2::pixels;

pub const ROWS: usize = 10;
pub const COLS: usize = 10;
pub const WIDTH: u32 = 500;
pub const HEIGHT: u32 = 500;

pub const TILE_WIDTH: u32 = WIDTH / COLS as u32;
pub const TILE_HEIGHT: u32 = HEIGHT / ROWS as u32;

pub const ACTIVE_COLOR: pixels::Color = pixels::Color::RGB(0, 255, 0);
pub const INACTIVE_COLOR: pixels::Color = pixels::Color::RGB(0, 0, 0);

pub const BORDER_COLOR: pixels::Color = pixels::Color::RGB(200, 200, 200);