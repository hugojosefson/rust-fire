pub const WIDTH: usize = 320 as usize;
pub const HEIGHT: usize = 200 as usize;
pub const GENERATOR_HEIGHT: usize = 2;
pub const GENERATOR_SIZE: usize = WIDTH * GENERATOR_HEIGHT;
pub const VISIBLE_HEIGHT: usize = HEIGHT - 5;

pub const WIDTH_U32: u32 = WIDTH as u32;
pub const VISIBLE_HEIGHT_U32: u32 = VISIBLE_HEIGHT as u32;

pub const DATA_SIZE: usize = WIDTH * HEIGHT;
pub const VISIBLE_DATA_SIZE: usize = WIDTH * VISIBLE_HEIGHT;
pub const PIXEL_DATA_SIZE: usize = VISIBLE_DATA_SIZE * 4;
