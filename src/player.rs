use crate::state::GameState;
use crate::{PLAYER_SPEED, WORLD_HEIGHT, WORLD_WIDTH};
use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub xp: u32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::Gaming)),
        );
    }
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
            transform.translation.x + delta.x * PLAYER_SPEED * time.delta_seconds(),
            -WORLD_WIDTH,
        )
    } else {
        f32::min(
            transform.translation.x + delta.x * PLAYER_SPEED * time.delta_seconds(),
            WORLD_WIDTH,
        )
    };
    transform.translation.y = if delta.y < 0.0 {
        f32::max(
            transform.translation.y + delta.y * PLAYER_SPEED * time.delta_seconds(),
            -WORLD_HEIGHT,
        )
    } else {
        f32::min(
            transform.translation.y + delta.y * PLAYER_SPEED * time.delta_seconds(),
            WORLD_HEIGHT,
        )
    };
    transform.translation.z = 10.0;
}
