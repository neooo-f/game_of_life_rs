use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas};

fn draw_grid(rows: i32, cols: i32, cell_size: i32, color: Color, canvas: &mut WindowCanvas) {
    // draw white background
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.set_draw_color(color);

    for row in 0..=rows {
        canvas.draw_line((0, row * cell_size), (cols * cell_size, row * cell_size)).expect("Failed to draw line");
    }

    for col in 0..=cols {
        canvas.draw_line((col * cell_size, 0), (col * cell_size, rows * cell_size)).expect("Failed to draw line");
    }
}

fn draw_block(row: i32, col: i32, cell_size: i32, color: Color, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(col * cell_size, row * cell_size, cell_size as u32, cell_size as u32)).expect("Failed to draw rectangle");
}

fn clear_canvas(width: u32, height: u32, color: Color, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(0, 0, width, height)).expect("Failed to clear the canvas");
}

fn draw_cells(board: &Vec<Vec<i32>>, cell_size: i32, color: Color, canvas: &mut WindowCanvas) {
    let num_rows = board.len();
    let num_cols = board[0].len();

    for row in 0..num_rows {
        for col in 0..num_cols {
            if board[row][col] == 1 {
                draw_block(row as i32, col as i32, cell_size, color, canvas);
            }
        }
    }
}

pub fn refresh_window(window_width: u32, window_height: u32, rows: i32, cols: i32, board: &Vec<Vec<i32>>, cell_size: i32, grid_color: Color, cell_color: Color, background_color: Color, canvas: &mut WindowCanvas) {
    clear_canvas(window_width, window_height, background_color, canvas);
    draw_grid(rows, cols, cell_size, grid_color, canvas);
    draw_cells(board, cell_size, cell_color, canvas);
}