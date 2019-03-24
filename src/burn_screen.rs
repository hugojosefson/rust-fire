use crate::constants::*;

pub fn burn_screen(data: &mut [u32]) {
    for i in WIDTH + 1..SCREEN_SIZE - WIDTH - 1 {
        data[i] = (data[i - 1]
            + data[i]
            + data[i + 1]
            + data[i + WIDTH - 1]
            + data[i + WIDTH + 1]
            + data[i + 2 * WIDTH - 1]
            + data[i + 2 * WIDTH]
            + data[i + 2 * WIDTH + 1])
            / 8;
    }
}
