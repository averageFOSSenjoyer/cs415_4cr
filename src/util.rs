use crate::config::CONFIG;

pub fn get_sprite_index(row_id: u32, col_id: u32) -> usize {
    assert!(row_id < CONFIG.sprite.spritesheet_height);
    assert!(col_id < CONFIG.sprite.spritesheet_width);

    (row_id * CONFIG.sprite.spritesheet_width + col_id) as usize
}
