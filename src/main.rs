#![feature(proc_macro_hygiene)]
extern crate flame;
extern crate sdl2;
#[macro_use]
extern crate flamer;

use rand::prelude::ThreadRng;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use std::slice::Iter;

const WIDTH_U32: u32 = 320;
const WIDTH: usize = WIDTH_U32 as usize;
const HEIGHT_U32: u32 = 200;
const DATA_SIZE: usize = (WIDTH_U32 * (HEIGHT_U32 + 1)) as usize;
const PIXEL_DATA_SIZE: usize = DATA_SIZE * 4 as usize;
const SCREEN_SIZE: usize = (WIDTH_U32 * HEIGHT_U32) as usize;

#[flame]
fn cycle_generator(rng: &mut ThreadRng, data: &mut Vec<u8>) {
    for i in SCREEN_SIZE..DATA_SIZE - 1 {
        if data[i] < 255 {
            data[i] = data[i] + 1
        } else {
            data[i] = rng.gen_range(64 + 16, 255)
        }
    }
}

#[flame]
fn burn_screen(data: &mut Vec<u8>) {
    for i in WIDTH + 1..SCREEN_SIZE - WIDTH - 2 {
        let up_left = data[i - 1];
        let up = data[i];
        let up_right = data[i + 1];
        let left = data[i + WIDTH - 1];
        let right = data[i + WIDTH + 1];
        let down_left = data[i + 2 * WIDTH - 1];
        let down = data[i + 2 * WIDTH];
        let down_right = data[i + 2 * WIDTH + 1];
        let burnt: u8 = ((up_left as u32
            + up as u32
            + up_right as u32
            + down_left as u32
            + down as u32
            + down_right as u32
            + left as u32
            + right as u32)
            / 8) as u8;
        data[i] = burnt;
    }
}

fn color_from_index(color_index: &u8) -> Vec<u8> {
    let color_index: u8 = *color_index;
    vec![
        color_index >> 2, // blue
        color_index >> 1, // green
        color_index >> 0, // red
        0u8,              // unused (alpha?)
    ]
}

#[flame]
fn color_indices_to_pixel_data(color_indices: &Vec<u8>, pixel_data: &mut [u8]) {
    let iterator: Iter<u8> = color_indices.iter();
    iterator
        .flat_map(color_from_index)
        .enumerate()
        .for_each(|(i, byte)| pixel_data[i] = byte)
}

#[flame]
fn draw(texture: &mut Texture, pixel_data: &[u8]) -> Result<(), String> {
    texture
        .update(None, pixel_data, WIDTH)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[flame]
fn fire() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("fire", WIDTH_U32, HEIGHT_U32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture: Texture = texture_creator
        .create_texture_streaming(None, WIDTH_U32, HEIGHT_U32)
        .map_err(|e| e.to_string())?;

    let mut data: Vec<u8> = Vec::with_capacity(DATA_SIZE);
    let mut pixel_data: [u8; PIXEL_DATA_SIZE] = [0; PIXEL_DATA_SIZE];
    for i in 0..DATA_SIZE - 1 {
        let color_index = if i < SCREEN_SIZE {
            0 + rng.gen_range(64 + 16, 255)
        } else {
            rng.gen_range(64 + 16, 255)
        };
        data.insert(i, color_index);
    }
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        cycle_generator(&mut rng, &mut data);
        burn_screen(&mut data);

        color_indices_to_pixel_data(&data, &mut pixel_data);
        draw(&mut texture, &pixel_data)?;
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    let result = fire();

    flame::dump_stdout();

    result
}
