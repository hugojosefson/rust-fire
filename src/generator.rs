use crate::constants::*;
use rand::prelude::ThreadRng;
use rand::Rng;

fn create_random_value(rng: &mut ThreadRng) -> u32 {
    rng.gen_range(0..256)
}

pub fn new(rng: &mut ThreadRng) -> [u32; GENERATOR_SIZE] {
    let mut generator: [u32; GENERATOR_SIZE] = [0u32; GENERATOR_SIZE];
    for i in generator.iter_mut().take(GENERATOR_SIZE) {
        *i = create_random_value(rng);
    }
    generator
}

// Increase each value in the generator by one.
// When a value hits the roof at 255, replace it with a new random value.
pub fn cycle(rng: &mut ThreadRng, generator: &mut [u32; GENERATOR_SIZE]) {
    for i in generator.iter_mut().take(GENERATOR_SIZE) {
        if *i == 0xff {
            *i = create_random_value(rng);
        } else {
            *i += 1;
        }
    }
}
