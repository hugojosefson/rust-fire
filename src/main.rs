extern crate sdl2;

use rand::prelude::ThreadRng;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const WIDTH_U32: u32 = 320;
const WIDTH: usize = WIDTH_U32 as usize;
const HEIGHT_U32: u32 = 200;
const DATA_SIZE: usize = (WIDTH_U32 * (HEIGHT_U32 + 1)) as usize;
const PIXEL_DATA_SIZE: usize = DATA_SIZE * 4 as usize;
const SCREEN_SIZE: usize = (WIDTH_U32 * HEIGHT_U32) as usize;

fn cycle_generator(rng: &mut ThreadRng, data: &mut [u32]) {
    for i in SCREEN_SIZE..DATA_SIZE - 1 {
        if data[i] < 255 {
            data[i] = data[i] + 1
        } else {
            data[i] = rng.gen_range(64 + 16, 255)
        }
    }
}

fn burn_screen(data: &mut [u32]) {
    let mut sum: u32;
    for i in WIDTH + 1..SCREEN_SIZE - WIDTH - 1 {
        sum = data[i - 1];
        sum += data[i];
        sum += data[i + 1];
        sum += data[i + WIDTH - 1];
        sum += data[i + WIDTH + 1];
        sum += data[i + 2 * WIDTH - 1];
        sum += data[i + 2 * WIDTH];
        sum += data[i + 2 * WIDTH + 1];
        sum /= 8;
        data[i] = sum
    }
}

fn color_indices_to_pixel_data(color_indices: &[u32], pixel_data: &mut [u8]) {
    color_indices
        .iter()
        .enumerate()
        .for_each(|(i, &color_index)| {
            let pixels: [u8; 4] = [
                (color_index >> 2) as u8,
                (color_index >> 1) as u8,
                (color_index) as u8,
                0u8,
            ];
            pixel_data[i * 4..i * 4 + 4].copy_from_slice(&pixels);
        });
}

fn draw_to_texture(texture: &mut Texture, pixel_data: &[u8]) -> Result<(), String> {
    texture
        .update(None, pixel_data, WIDTH * 4)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn draw_to_canvas(canvas: &mut Canvas<Window>, texture: &mut Texture) -> Result<(), String> {
    canvas.copy(&texture, None, None)
}

fn fire() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("fire", WIDTH_U32, HEIGHT_U32)
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture: Texture = texture_creator
        .create_texture_streaming(None, WIDTH_U32, HEIGHT_U32)
        .map_err(|e| e.to_string())?;

    let mut data: [u32; DATA_SIZE] = [0; DATA_SIZE];
    let mut pixel_data: [u8; PIXEL_DATA_SIZE] = [0; PIXEL_DATA_SIZE];
    for i in SCREEN_SIZE..DATA_SIZE {
        data[i] = rng.gen_range(64 + 16, 255);
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
        draw_to_texture(&mut texture, &pixel_data)?;
        draw_to_canvas(&mut canvas, &mut texture)?;
        canvas.present();
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    fire()
}
