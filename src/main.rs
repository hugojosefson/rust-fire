#![feature(fixed_size_array)]
extern crate sdl2;

use ndarray::{Array2, ShapeBuilder};
use ndarray::Dimension;
use ndarray::Shape;
use rand::Rng;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const WIDTH: u32 = 800;
const WIDTH_I16: i16 = 800;
const HEIGHT: u32 = 600;
const DATA_SIZE: usize = (WIDTH * (HEIGHT + 1)) as usize;
const SCREEN_SIZE: usize = (WIDTH * HEIGHT) as usize;
const SCREEN_SIZE_MINUS_1: usize = SCREEN_SIZE - 1;

fn color_from_index(i: u8) -> Color {
    Color::from((i, i >> 1, i >> 2))
}

struct Cell {
    x: usize,
    y: usize,
    color_index: u8,
}

pub fn main() {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("fire", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let shape =
    let f = |(x, y)| Cell {
        x,
        y,
        color_index: rng.gen_range(0, 255),
    };
    let mut data = Array2::<Cell>::from_shape_fn(shape, f);

    data.iter().for_each(|cell: &Cell| {
        canvas.pixel(
            cell.x as i16,
            cell.y as i16,
            color_from_index(cell.color_index),
        );
    });

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;

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

        canvas.present();
    }
}
