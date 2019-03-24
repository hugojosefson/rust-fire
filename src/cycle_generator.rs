use crate::constants::*;
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn cycle_generator(rng: &mut ThreadRng, data: &mut [u32]) {
    for i in SCREEN_SIZE..DATA_SIZE - 1 {
        if data[i] < 255 {
            data[i] = data[i] + 1
        } else {
            data[i] = rng.gen_range(64 + 16, 255)
        }
    }
}
