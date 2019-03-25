use crate::constants::*;

fn get_value_from(
    data: &[u32],
    generator: &[u32; GENERATOR_SIZE],
    index: usize,
    offset: isize,
) -> u32 {
    let index: isize = index as isize + offset;
    if index < DATA_SIZE as isize {
        let index: usize = index.clamp(0, (DATA_SIZE - 1) as isize) as usize;
        data[index]
    } else {
        let index = index - DATA_SIZE as isize;
        let index: usize = index.clamp(0, (GENERATOR_SIZE - 1) as isize) as usize;
        generator[index]
    }
}

pub fn burn_screen(data: &mut [u32], generator: &[u32; GENERATOR_SIZE]) {
    for i in 0..DATA_SIZE {
        let mut value = get_value_from(data, generator, i, -1)
            + get_value_from(data, generator, i, 0)
            + get_value_from(data, generator, i, 1)
            + get_value_from(data, generator, i, (WIDTH - 1) as isize)
            + get_value_from(data, generator, i, (WIDTH + 1) as isize)
            + get_value_from(data, generator, i, (2 * WIDTH - 1) as isize)
            + get_value_from(data, generator, i, (2 * WIDTH) as isize)
            + get_value_from(data, generator, i, (2 * WIDTH + 1) as isize);

        value = if value > BURN_SPEED * 2 {
            value - BURN_SPEED
        } else {
            0
        };

        value /= 8;

        data[i] = value;
    }
}
