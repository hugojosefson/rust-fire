use crate::constants::*;

pub fn new() -> [[u8; 4]; PALETTE_SIZE] {
    let mut palette_array: [[u8; 4]; PALETTE_SIZE] = [[0u8; 4]; PALETTE_SIZE];
    for color_index in 0..PALETTE_SIZE {
        let pixel: [u8; 4] = [
            (color_index >> 2) as u8,
            (color_index >> 1) as u8,
            (color_index) as u8,
            0u8,
        ];
        palette_array[color_index] = pixel;
    }
    return palette_array;
}
