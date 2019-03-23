#![feature(fixed_size_array)]
extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 800;
const WIDTH_USIZE: usize = 800;
const HEIGHT: u32 = 600;
const DATA_SIZE: usize = (WIDTH * (HEIGHT + 1)) as usize;
const SCREEN_SIZE: usize = (WIDTH * HEIGHT) as usize;

fn color_from_index(i: u8) -> Color {
    Color::from((i, i >> 1, i >> 2))
}

struct Cell {
    x: usize,
    y: usize,
    color_index: u8,
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
        .window("fire", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut data: Vec<Cell> = Vec::with_capacity(DATA_SIZE);

    for pixel_index in 0..DATA_SIZE - 1 {
        let x = pixel_index % WIDTH_USIZE;
        let y = pixel_index / WIDTH_USIZE;
        data.insert(
            pixel_index,
            Cell {
                x,
                y,
                color_index: if pixel_index < SCREEN_SIZE {
                    0
                } else {
                    rng.gen_range(0, 255)
                },
            },
        );
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i: u8 = 0;
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

        i = i.wrapping_add(1);
        draw(&canvas, &data)?;

        canvas.present();
    }
    Ok(())
}
