pub const WINDOW_HEIGHT: f32 = 1200.0;
pub const WINDOW_WIDTH: f32 = 800.0;
pub const SPRITE_HEIGHT: u32 = 32;
pub const SPRITE_WIDTH: u32 = 32;
pub const SPRITE_SCALE_FACTOR: f32 = 1.5;
pub const SPRITESHEET_PATH: &str = "sprites.png";
pub const SPRITESHEET_HEIGHT: u32 = 8;
pub const SPRITESHEET_WIDTH: u32 = 16;
pub const WORLD_WIDTH: f32 = 2500.0;
pub const WORLD_HEIGHT: f32 = 2500.0;
pub const BACKGROUND_COLOR: (u8, u8, u8) = (163, 116, 46);
pub const DECORATIONS_DENSITY: f32 = 0.00027777777;
pub const NUM_DECORATIONS: usize = (WORLD_WIDTH * WORLD_HEIGHT * DECORATIONS_DENSITY) as usize;
pub const PLAYER_SPEED: f32 = 175.0;
pub const ATTACK_INTERVAL: f32 = 1.0;
pub const PROJECTILE_SPEED: f32 = 600.0;
pub const MAX_NUM_ENEMIES: usize = 50;
pub const ENEMY_SPAWN_INTERVAL: f32 = 2.0;
pub const ENEMY_SPEED: f32 = 125.0;
pub const ENEMY_HEALTH: f32 = 1.0;
pub const ANIMATION_TICK_DURATION: f32 = 0.1;
pub const PROJECTILE_DAMAGE: f32 = 1.0;
