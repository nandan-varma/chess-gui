use raylib::prelude::*;

const SELECTION_COLOR: Color = Color {
    r: 0,
    g: 121,
    b: 241,
    a: 100,
};

pub struct Board {
    m_board: [[i32; 8]; 8],
}

pub fn build_board() -> Board {
    Board {
        m_board: [[0; 8]; 8],
    }
}


pub fn draw_board( draw_handle:&mut RaylibDrawHandle, _board: &Board, _selections: &Vec<(i32, i32)>, block_size: i32) -> () {
    for x in 0..8 {
        for y in 0..8 {
            if (x + y) % 2 == 1 {
                draw_handle.draw_rectangle(
                    x * block_size,
                    y * block_size,
                    block_size,
                    block_size,
                    Color::BLACK,
                );
            } else {
                draw_handle.draw_rectangle(
                    x * block_size,
                    y * block_size,
                    block_size,
                    block_size,
                    Color::WHITE,
                );
            }
        }
    }
    for z in _selections {
        let (x, y) = z;
        draw_handle.draw_rectangle(
            x * block_size,
            y * block_size,
            block_size,
            block_size,
            SELECTION_COLOR,
        );
    }
}
