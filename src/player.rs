use crate::animation::AnimationTimer;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::util::get_sprite_index;
use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::*;
use crate::config::CONFIG;

#[derive(Component)]
pub struct Player {
    pub xp: u32,
    pub health: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { xp: 0, health: 1.0 }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_player.run_if(in_state(GameState::Initializing))
        ).add_systems(
            Update,
            (handle_player_input, check_player_death, handle_player_xp).run_if(in_state(GameState::Gaming)),
        );
    }
}

fn init_player(
    mut commands: Commands,
    texture_handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(CONFIG.sprite.sprite_scale_factor)),
            ..default()
        },
        TextureAtlas {
            layout: texture_handle.layout.clone().unwrap(),
            index: get_sprite_index(0, 0),
        },
        Player::default(),
        AnimationTimer(Timer::from_seconds(
            CONFIG.game.animation_tick_interval,
            TimerMode::Repeating,
        )),
    ));

    next_state.set(GameState::Gaming);
}

fn handle_player_input(
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut();
    let w_key_pressed = keyboard_input.pressed(KeyCode::KeyW);
    let a_key_pressed = keyboard_input.pressed(KeyCode::KeyA);
    let s_key_pressed = keyboard_input.pressed(KeyCode::KeyS);
    let d_key_pressed = keyboard_input.pressed(KeyCode::KeyD);

    let mut delta = Vec2::ZERO;
    if w_key_pressed {
        delta.y += 1.0;
    }
    if s_key_pressed {
        delta.y -= 1.0;
    }
    if a_key_pressed {
        delta.x -= 1.0;
    }
    if d_key_pressed {
        delta.x += 1.0;
    }
    delta = delta.normalize_or_zero();
    transform.translation.x = if delta.x < 0.0 {
        f32::max(
            transform.translation.x + delta.x * CONFIG.player.movement_speed * time.delta_seconds(),
            -CONFIG.game.world_width,
        )
    } else {
        f32::min(
            transform.translation.x + delta.x * CONFIG.player.movement_speed * time.delta_seconds(),
            CONFIG.game.world_width,
        )
    };
    transform.translation.y = if delta.y < 0.0 {
        f32::max(
            transform.translation.y + delta.y * CONFIG.player.movement_speed * time.delta_seconds(),
            -CONFIG.game.world_height,
        )
    } else {
        f32::min(
            transform.translation.y + delta.y * CONFIG.player.movement_speed * time.delta_seconds(),
            CONFIG.game.world_height,
        )
    };
    transform.translation.z = 10.0;
}

fn check_player_death(
    player_query: Query<&Player, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let player = player_query.single();
    if player.health <= 0.0 {
        next_state.set(GameState::Dying);
    }
}

fn handle_player_xp(
    mut player_query: Query<&mut Player, With<Player>>,
) {
    for mut player in player_query.iter_mut() {
        if player.xp >= 10 {
            player.xp = 0;

        }
    }
}