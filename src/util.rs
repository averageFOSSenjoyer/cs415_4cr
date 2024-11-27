use crate::{SPRITESHEET_HEIGHT, SPRITESHEET_WIDTH};

pub fn get_sprite_index(row_id: u32, col_id: u32) -> usize {
    assert!(row_id < SPRITESHEET_HEIGHT);
    assert!(col_id < SPRITESHEET_WIDTH);

    (row_id * SPRITESHEET_WIDTH + col_id) as usize
}
