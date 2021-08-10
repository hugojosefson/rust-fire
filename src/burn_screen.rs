use crate::constants::*;

fn clamp(index: isize, max: usize) -> usize {
    if index < 0 {
        return 0;
    }

    if index > max as isize {
        max
    } else {
        index as usize
    }
}

fn get_value_from(
    data: &[u32],
    generator: &[u32; GENERATOR_SIZE],
    index: usize,
    offset: isize,
) -> u32 {
    let index: isize = index as isize + offset;
    if index < DATA_SIZE as isize {
        let index: usize = clamp(index, DATA_SIZE - 1);
        data[index]
    } else {
        let index = index - DATA_SIZE as isize;
        let index: usize = clamp(index, GENERATOR_SIZE - 1);
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
