use crate::constants::*;
use rand::prelude::ThreadRng;
use rand::Rng;

fn create_random_value(rng: &mut ThreadRng) -> u32 {
    rng.gen_range(0, 255)
}

pub fn new(rng: &mut ThreadRng) -> [u32; WIDTH] {
    let mut generator: [u32; WIDTH] = [0u32; WIDTH];
    for i in 0..WIDTH {
        generator[i] = create_random_value(rng);
    }
    generator
}

pub fn cycle(rng: &mut ThreadRng, generator: &mut [u32; WIDTH]) {
    for i in 0..WIDTH {
        let value = generator[i];
        if value < 255 {
            generator[i] = value + 1;
        } else {
            generator[i] = create_random_value(rng);
        }
    }
}
