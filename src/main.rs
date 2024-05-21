mod draw;
mod logic;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use rand::Rng;
use crate::draw::refresh_window;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // constants
    const WINDOW_WIDTH: u32 = 2000; // -> 1600
    const WINDOW_HEIGHT: u32 = 1200; // -> 1000
    const WINDOW_TITLE: &str = "Game Of Life - Rust SDL2";

    const GRID_CELL_SIZE: i32 = 3; // -> 50
    const BACKGROUND_COLOR: Color = Color::RGB(255, 255, 255); // white
    const GRID_COLOR: Color = Color::RGB(128, 128, 128); // grey
    const BLOCK_COLOR: Color = Color::RGB(0, 0, 0); // black
    const GRID_ROWS: i32 = (WINDOW_HEIGHT as i32 + GRID_CELL_SIZE - 1) / GRID_CELL_SIZE;
    const GRID_COLS: i32 = (WINDOW_WIDTH as i32 + GRID_CELL_SIZE - 1) / GRID_CELL_SIZE;
    const ITERATION_TIMEOUT_MILLS: i16 = 0; // -> 500

    let mut _board: Vec<Vec<i32>> = vec![vec![0; GRID_COLS as usize]; GRID_ROWS as usize];

    // random board with range
    let mut rng = rand::thread_rng();

    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            if (row + col) as f64 % 1.5 == 0.0 {
                _board[row as usize][col as usize] = if rng.gen::<f64>() < 0.5 { 0 } else { 1 };
            } else {
                _board[row as usize][col as usize] = 0;
            }
        }
    }

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut animate = false;

    // main (animation) loop
    'running: loop {

        // recognizes keyboard events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    animate = !animate;
                }
                Event::MouseButtonDown {
                    mouse_btn,
                    x,
                    y,
                    ..
                } if !animate => {
                    logic::set_block(mouse_btn, x, y, GRID_CELL_SIZE, &mut _board);
                }
                _ => {}
            }
        }

        refresh_window(WINDOW_WIDTH, WINDOW_HEIGHT, GRID_ROWS, GRID_COLS, &_board, GRID_CELL_SIZE, GRID_COLOR, BLOCK_COLOR, BACKGROUND_COLOR, &mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        if animate {
            let mut next_board = _board.clone();
            logic::check_cells(&mut _board, &mut next_board);
            _board = next_board;

            ::std::thread::sleep(Duration::from_millis(ITERATION_TIMEOUT_MILLS as u64));
        }
    }

    Ok(())
}