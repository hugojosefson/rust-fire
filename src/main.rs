#![feature(proc_macro_hygiene)]
extern crate flame;
extern crate sdl2;
#[macro_use]
extern crate flamer;

use rand::prelude::ThreadRng;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const WIDTH_U32: u32 = 320;
const WIDTH: usize = WIDTH_U32 as usize;
const HEIGHT_U32: u32 = 200;
const DATA_SIZE: usize = (WIDTH_U32 * (HEIGHT_U32 + 1)) as usize;
const SCREEN_SIZE: usize = (WIDTH_U32 * HEIGHT_U32) as usize;

fn color_from_index(ci: u8) -> Color {
    Color::from((ci, ci >> 1, ci >> 2))
}

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

#[flame]
fn draw(canvas: &mut Canvas<Window>, data: &Vec<u8>) -> Result<(), String> {
    let pixel_data: Vec<u8> = data
        .iter()
        .flat_map(|&i| {
            let color = color_from_index(i);
            [
                color.r.clone(),
                color.g.clone(),
                color.b.clone(),
                color.a.clone(),
            ]
            .iter()
        })
        .map(|&byte| byte)
        .collect();

    let texture: Texture = canvas
        .texture_creator()
        .create_texture_target(None, WIDTH_U32, HEIGHT_U32)
        .map_err(|e| e.to_string())?;

    texture.update(None, pixel_data.as_slice(), WIDTH);
    Ok(())
}

#[flame]
fn fire() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("fire", WIDTH_U32 * 2 + 1, HEIGHT_U32 * 2 + 1)
        .fullscreen()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut data: Vec<u8> = Vec::with_capacity(DATA_SIZE);
    for i in 0..DATA_SIZE - 1 {
        let color_index = if i < SCREEN_SIZE {
            0
        } else {
            rng.gen_range(0, 255)
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
        draw(&mut canvas, &data)?;

        canvas.present();
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    let result = fire();

    //    flame::dump_html(&mut File::create("target/flame-graph.html").unwrap()).unwrap();
    flame::dump_stdout();

    result
}
