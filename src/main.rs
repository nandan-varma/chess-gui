mod board;
mod movement;
use board::*;
use movement::*;
use raylib::prelude::*;

fn main() {
    let board = build_board();
    let mut selections: Vec<(i32, i32)> = Vec::new();
    // let mut selection :(i32,i32) = (0,0);
    let block_size = 100;
    let window_height = block_size * 8;
    let window_width = block_size * 8;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Sudoku")
        .build();

    // rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        draw_board(&mut d, &board, &selections, block_size);
        handle_selection(&mut d, &mut selections, block_size);
        // d.draw_fps(0, 0);
        // d.draw_text("Test 1234567890", 12, 12, 20, Color::BLACK);
    }
}
