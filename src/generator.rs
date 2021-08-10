use crate::constants::*;
use rand::prelude::ThreadRng;
use rand::Rng;

fn create_random_value(rng: &mut ThreadRng) -> u32 {
    rng.gen_range(0, 255)
}

//noinspection RsExternalLinter
pub fn new(rng: &mut ThreadRng) -> [u32; GENERATOR_SIZE] {
    let mut generator: [u32; GENERATOR_SIZE] = [0u32; GENERATOR_SIZE];
    for i in 0..GENERATOR_SIZE {
        generator[i] = create_random_value(rng);
    }
    generator
}

//noinspection RsExternalLinter
pub fn cycle(rng: &mut ThreadRng, generator: &mut [u32; GENERATOR_SIZE]) {
    for i in 0..GENERATOR_SIZE {
        let value = generator[i];
        if value < 255 {
            generator[i] = value + 1;
        } else {
            generator[i] = create_random_value(rng);
        }
    }
}
