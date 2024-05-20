use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};

pub fn draw_grid(rows: i32, cols: i32, cell_size: i32, color: Color, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(color);

    for row in 0..=rows {
        canvas.draw_line((0, row * cell_size), (cols * cell_size, row * cell_size)).expect("Failed to draw line");
    }

    for col in 0..=cols {
        canvas.draw_line((col * cell_size, 0), (col * cell_size, rows * cell_size)).expect("Failed to draw line");
    }
}