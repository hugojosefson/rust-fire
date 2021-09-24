use crate::constants::*;
use rand::prelude::ThreadRng;
use rand::Rng;

fn create_random_value(rng: &mut ThreadRng) -> u32 {
    rng.gen_range(0, 255)
}

pub fn new(rng: &mut ThreadRng) -> [u32; GENERATOR_SIZE] {
    let mut generator: [u32; GENERATOR_SIZE] = [0u32; GENERATOR_SIZE];
    for i in generator.iter_mut().take(GENERATOR_SIZE) {
        *i = create_random_value(rng);
    }
    generator
}

pub fn cycle(rng: &mut ThreadRng, generator: &mut [u32; GENERATOR_SIZE]) {
    for i in generator.iter_mut().take(GENERATOR_SIZE) {
        let value = *i;
        if value < 255 {
            *i = value + 1;
        } else {
            *i = create_random_value(rng);
        }
    }
}
