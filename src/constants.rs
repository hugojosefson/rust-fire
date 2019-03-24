pub const WIDTH_U32: u32 = 320;
pub const HEIGHT_U32: u32 = 200;

pub const WIDTH: usize = WIDTH_U32 as usize;
pub const DATA_SIZE: usize = (WIDTH_U32 * (HEIGHT_U32 + 1)) as usize;
pub const PIXEL_DATA_SIZE: usize = DATA_SIZE * 4 as usize;
pub const SCREEN_SIZE: usize = (WIDTH_U32 * HEIGHT_U32) as usize;
