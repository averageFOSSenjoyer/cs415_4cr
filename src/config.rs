use std::env;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: Config = if let Ok(config_file_str) = env::var("CONFIG_FILE") {
        if let Ok(config_str) = std::fs::read_to_string(config_file_str) {
            toml::from_str(&config_str).unwrap()
        } else {
            println!("Cannot parse given config, using defaults");
            Config::default()
        }
    } else {
        println!("Config not provided or not found, using defaults");
        Config::default()
    };
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub app: AppConfig,
    pub game: GameConfig,
    pub sprite: SpriteConfig,
    pub player: PlayerConfig,
    pub enemy: EnemyConfig,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub window_height: f32,
    pub window_width: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_height: 1200.0,
            window_width: 800.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub world_width: f32,
    pub world_height: f32,
    pub background_color: (u8, u8, u8),
    pub decoration_density: f32,
    pub animation_tick_interval: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            world_width: 2500.0,
            world_height: 2500.0,
            background_color: (163, 116, 46),
            decoration_density: 0.00027777777,
            animation_tick_interval: 0.1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpriteConfig {
    pub spritesheet_path: String,
    pub spritesheet_height: u32,
    pub spritesheet_width: u32,
    pub sprite_height: u32,
    pub sprite_width: u32,
    pub sprite_scale_factor: f32,
}

impl Default for SpriteConfig {
    fn default() -> Self {
        Self {
            spritesheet_path: "sprites.png".to_string(),
            spritesheet_height: 8,
            spritesheet_width: 16,
            sprite_height: 32,
            sprite_width: 32,
            sprite_scale_factor: 1.5,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerConfig {
    pub movement_speed: f32,
    pub attack_interval: f32,
    pub projectile_speed: f32,
    pub projectile_damage: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            movement_speed: 175.0,
            attack_interval: 1.0,
            projectile_speed: 600.0,
            projectile_damage: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnemyConfig {
    pub max_num_enemies: usize,
    pub enemy_spawn_interval: f32,
    pub enemy_health: f32,
    pub enemy_speed: f32,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        Self {
            max_num_enemies: 50,
            enemy_spawn_interval: 2.0,
            enemy_health: 1.0,
            enemy_speed: 125.0,
        }
    }
}