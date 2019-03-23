#![feature(fixed_size_array)]
extern crate sdl2;

use rand::prelude::ThreadRng;
use rand::Rng;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH_U32: u32 = 320;
const WIDTH: usize = WIDTH_U32 as usize;
const HEIGHT_U32: u32 = 200;
const DATA_SIZE: usize = (WIDTH_U32 * (HEIGHT_U32 + 1)) as usize;
const SCREEN_SIZE: usize = (WIDTH_U32 * HEIGHT_U32) as usize;

fn color_from_index(i: u8) -> Color {
    Color::from((i, i >> 1, i >> 2))
}

struct Cell {
    x: usize,
    y: usize,
    color_index: u8,
}

fn cycle_generator(rng: &mut ThreadRng, data: &mut Vec<Cell>) {
    for mut cell in &mut data[SCREEN_SIZE..] {
        cell.color_index = if cell.color_index < 255 {
            cell.color_index.wrapping_add(1)
        } else {
            rng.gen_range(0, 255)
        };
    }
}

fn draw(canvas: &Canvas<Window>, data: &Vec<Cell>) -> Result<(), String> {
    for cell in data {
        canvas.pixel(
            cell.x as i16,
            cell.y as i16,
            color_from_index(cell.color_index),
        )?;
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("fire", WIDTH_U32, HEIGHT_U32)
        .fullscreen()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut data: Vec<Cell> = Vec::with_capacity(DATA_SIZE);

    for pixel_index in 0..DATA_SIZE - 1 {
        let x = pixel_index % WIDTH;
        let y = pixel_index / WIDTH;
        let color_index = if pixel_index < SCREEN_SIZE {
            0
        } else {
            rng.gen_range(0, 255)
        };
        data.insert(pixel_index, Cell { x, y, color_index });
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
        for pixel_index in WIDTH + 1..SCREEN_SIZE - WIDTH - 2 {
            let up_left = data[pixel_index + WIDTH - WIDTH - 1].color_index;
            let up = data[pixel_index + WIDTH - WIDTH].color_index;
            let up_right = data[pixel_index + WIDTH - WIDTH + 1].color_index;
            let down_left = data[pixel_index + WIDTH + WIDTH - 1].color_index;
            let down = data[pixel_index + WIDTH + WIDTH].color_index;
            let down_right = data[pixel_index + WIDTH + WIDTH + 1].color_index;
            let left = data[pixel_index + WIDTH - 1].color_index;
            let right = data[pixel_index + WIDTH + 1].color_index;
            let color_index: u8 = ((up_left as u32
                + up as u32
                + up_right as u32
                + down_left as u32
                + down as u32
                + down_right as u32
                + left as u32
                + right as u32)
                / 8) as u8;
            data[pixel_index].color_index = color_index;
        }
        draw(&canvas, &data)?;

        canvas.present();
    }
    Ok(())
}
