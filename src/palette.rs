use crate::constants::*;

fn clamp(f: f32) -> u8 {
    if f > 255f32 {
        255
    } else {
        f as u8
    }
}

pub fn new() -> [[u8; 4]; PALETTE_SIZE] {
    let mut palette_array: [[u8; 4]; PALETTE_SIZE] = [[0u8; 4]; PALETTE_SIZE];
    for (color_index, palette_slot) in palette_array.iter_mut().enumerate().take(PALETTE_SIZE) {
        let f: f32 = color_index as f32;
        let red = f * 1.2;
        let green = f / 2.5;
        let blue = f / 5.;
        let pixel: [u8; 4] = [clamp(blue), clamp(green), clamp(red), 0u8];
        *palette_slot = pixel;
    }
    palette_array
}
