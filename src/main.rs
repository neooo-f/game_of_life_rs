mod draw;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // constants
    const WINDOW_WIDTH: u32 = 1600;
    const WINDOW_HEIGHT: u32 = 1000;
    const WINDOW_TITLE: &str = "Game Of Life - Rust SDL2";

    const GRID_CELL_SIZE: i32 = 50;
    const GRID_COLOR: Color = Color::RGB(128, 128, 128); // grey
    const BLOCK_COLOR: Color = Color::RGB(0, 0, 0); // black
    const GRID_ROWS: i32 = (WINDOW_HEIGHT as i32 + GRID_CELL_SIZE - 1) / GRID_CELL_SIZE;
    const GRID_COLS: i32 = (WINDOW_WIDTH as i32 + GRID_CELL_SIZE - 1) / GRID_CELL_SIZE;
    const ITERATION_TIMEOUT: i16 = 0;

    // println!("{:?} GR, {:?} GC", GRID_ROWS, GRID_COLS);

    // TODO: create array representing board (fields)

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

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
                _ => {}
            }
        }

        // white background
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // drawing color for rect
        // canvas.set_draw_color(Color::RGB(255, 0, 0)); // Red color

        // draw the rect
        // let rect = Rect::new(100, 100, 200, 200);
        // canvas.draw_rect(rect)?;

        draw::draw_grid(GRID_ROWS, GRID_COLS, GRID_CELL_SIZE, GRID_COLOR, &mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
        // println!("i am here in loop!");
    }

    Ok(())
}