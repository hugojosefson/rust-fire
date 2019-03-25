use crate::constants::*;

pub fn new() -> [[u8; 4]; PALETTE_SIZE] {
    let mut palette_array: [[u8; 4]; PALETTE_SIZE] = [[0u8; 4]; PALETTE_SIZE];
    for color_index in 0..PALETTE_SIZE {
        let f: f32 = color_index as f32;
        let red = f * 1.2;
        let green = f / 2.5;
        let blue = f / 5.;
        let pixel: [u8; 4] = [blue as u8, green as u8, red as u8, 0u8];
        palette_array[color_index] = pixel;
    }
    return palette_array;
}
