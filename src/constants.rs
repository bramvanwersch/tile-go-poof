use sdl2::pixels;

pub const ROWS: usize = 10;
pub const COLS: usize = 10;
pub const PLAY_WIDTH: u32 = 800;
pub const PLAY_HEIGHT: u32 = 800;
pub const BOTTOM_BAR: u32 = 30;
pub const HEIGHT: u32 = PLAY_HEIGHT + BOTTOM_BAR;
pub const WIDTH: u32 = PLAY_WIDTH;

pub const START_DIFFICULTY: usize = 4;

pub const MAX_TIME: u32 = 50;

pub const TEXT_COLOR: pixels::Color = pixels::Color::RGB(132, 222, 2);

pub const TILE_WIDTH: u32 = PLAY_WIDTH / COLS as u32;
pub const TILE_HEIGHT: u32 = PLAY_HEIGHT / ROWS as u32;

pub const ACTIVE_COLOR: pixels::Color = TEXT_COLOR;
pub const INACTIVE_COLOR: pixels::Color = pixels::Color::RGB(0, 0, 0);

pub const BORDER_COLOR: pixels::Color = pixels::Color::RGB(100, 100, 100);