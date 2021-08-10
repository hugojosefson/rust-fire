extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Mod;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
mod burn_screen;
mod constants;
mod generator;
mod palette;
use crate::burn_screen::burn_screen;
use crate::constants::*;

fn color_indices_to_pixel_data(
    palette: &[[u8; 4]; PALETTE_SIZE],
    color_indices: &[u32],
    pixel_data: &mut [u8],
) {
    for i in 0..color_indices.len() {
        let color_index = color_indices[i];
        let pixel: [u8; 4] = palette[color_index as usize];
        pixel_data[i * 4..i * 4 + 4].copy_from_slice(&pixel);
    }
}

fn draw_to_texture(texture: &mut Texture, pixel_data: &[u8]) -> Result<(), String> {
    texture
        .update(None, pixel_data, WIDTH * 4)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn draw_to_canvas(canvas: &mut Canvas<Window>, texture: &mut Texture) -> Result<(), String> {
    canvas.copy(texture, None, None)
}

fn is_bitmask_set(bitmask: u32, value: u32) -> bool {
    value & bitmask == bitmask
}

fn is_maximized(window: &Window) -> bool {
    is_bitmask_set(128, window.window_flags())
}

fn toggle_maximize(win: &mut Window) {
    if is_maximized(win) {
        win.restore()
    } else {
        win.maximize()
    }
}

fn fire() -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("fire", WIDTH_U32, VISIBLE_HEIGHT_U32)
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture: Texture = texture_creator
        .create_texture_streaming(None, WIDTH_U32, VISIBLE_HEIGHT_U32)
        .map_err(|e| e.to_string())?;

    let palette = palette::new();
    let mut generator: [u32; GENERATOR_SIZE] = generator::new(&mut rng);
    let mut data: [u32; DATA_SIZE] = [0; DATA_SIZE];
    let mut pixel_data: [u8; PIXEL_DATA_SIZE] = [0; PIXEL_DATA_SIZE];
    let mut event_pump = sdl_context.event_pump().unwrap();

    let (x, y) = canvas.window().size();
    let mut window_size = (x as i32, y as i32);
    let mut is_mouse_down: bool = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    keymod,
                    ..
                } => {
                    if keymod == Mod::LALTMOD | Mod::NUMMOD {
                        toggle_maximize(canvas.window_mut());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::KpEnter),
                    keymod,
                    ..
                } => {
                    if keymod == Mod::LALTMOD | Mod::NUMMOD {
                        toggle_maximize(canvas.window_mut());
                    }
                }
                Event::Window {
                    win_event: WindowEvent::Resized(x, y),
                    ..
                } => {
                    window_size = (x, y);
                }
                Event::MouseButtonDown { x, y, .. } => {
                    is_mouse_down = true;
                    paint_block(&mut data, &window_size, x, y);
                }
                Event::MouseButtonUp { .. } => {
                    is_mouse_down = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    if is_mouse_down {
                        paint_block(&mut data, &window_size, x, y);
                    }
                }
                _ => {}
            }
        }

        generator::cycle(&mut rng, &mut generator);
        burn_screen(&mut data, &generator);

        color_indices_to_pixel_data(&palette, &data[0..VISIBLE_DATA_SIZE], &mut pixel_data);
        draw_to_texture(&mut texture, &pixel_data)?;
        draw_to_canvas(&mut canvas, &mut texture)?;
        canvas.present();
    }
    Ok(())
}

fn paint_block(data: &mut [u32; DATA_SIZE], window_size: &(i32, i32), x: i32, y: i32) {
    let data_x = x as usize * WIDTH / window_size.0 as usize;
    let data_y = y as usize * VISIBLE_HEIGHT / window_size.1 as usize;
    for dx in 0usize..7usize {
        for dy in 0usize..7usize {
            paint_pixel(data, data_x + dx, data_y + dy);
        }
    }
}

fn paint_pixel(data: &mut [u32; DATA_SIZE], x: usize, y: usize) {
    let index = x + y * WIDTH;
    if index < DATA_SIZE {
        data[index] = 0xffu32;
    }
}

pub fn main() -> Result<(), String> {
    fire()
}
