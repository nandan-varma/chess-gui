use raylib::prelude::*;

pub fn get_selection(draw_handle: &mut RaylibDrawHandle , block_size : i32) -> (i32, i32) {
    (draw_handle.get_mouse_x() / block_size, draw_handle.get_mouse_y() / block_size)
}

pub fn handle_selection(draw_handle: &mut RaylibDrawHandle , _selections: &mut Vec<(i32, i32)> , block_size : i32) {
    if draw_handle.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)  {
        _selections.push(get_selection(draw_handle,block_size));
        println!("{:?}",_selections);
    }
}
