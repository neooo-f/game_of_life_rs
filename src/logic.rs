use sdl2::mouse::MouseButton;

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (1, 0),
    (1, -1),
    (1, 1),
];

fn get_wrapped_index(index: i32, max: i32) -> i32 {
    if index < 0 {
        return max - 1;
    }

    if index >= max {
        return 0;
    }

    index
}

pub fn set_block(mouse_button: MouseButton, x: i32, y: i32, cell_size: i32, board: &mut Vec<Vec<i32>>) {
    if mouse_button == MouseButton::Left {
        let row = y / cell_size;
        let col = x / cell_size;

        if board[row as usize][col as usize] == 1 {
            board[row as usize][col as usize] = 0;
        } else {
            board[row as usize][col as usize] = 1;
        }
    }
}

// game algorithm
pub fn check_cells(board: &mut Vec<Vec<i32>>, next_board: &mut Vec<Vec<i32>>) {
    let rows = board.len();
    let cols = board[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let mut neighbor_counter: i8 = 0;

            for &(dx, dy) in &NEIGHBOR_OFFSETS {
                let new_row = get_wrapped_index(row as i32 + dx, rows as i32);
                let new_col = get_wrapped_index(col as i32 + dy, cols as i32);

                if board[new_row as usize][new_col as usize] == 1 {
                    neighbor_counter += 1;
                }
            }

            // rules
            if neighbor_counter == 3 {
                next_board[row][col] = 1;
            }

            if neighbor_counter >= 4 || neighbor_counter == 1 || neighbor_counter == 0 {
                next_board[row][col] = 0;
            }
        }
    }
}

