use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::util::get_sprite_index;
use crate::weapon::{Weapon, WeaponTimer};
use crate::{
    ANIMATION_TICK_DURATION, NUM_DECORATIONS, SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH,
};
use bevy::app::{App, Plugin};
use bevy::math::{vec3, Vec3};
use bevy::prelude::*;
use bevy::time::{Stopwatch, Timer, TimerMode};
use rand::Rng;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Initializing),
            (init_world, decorate_world),
        );
    }
}

fn init_world(
    mut commands: Commands,
    texture_handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_handle.layout.clone().unwrap(),
            index: get_sprite_index(0, 0),
        },
        Player::default(),
        AnimationTimer(Timer::from_seconds(
            ANIMATION_TICK_DURATION,
            TimerMode::Repeating,
        )),
    ));

    commands.spawn((
        SpriteBundle {
            texture: texture_handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_handle.layout.clone().unwrap(),
            index: get_sprite_index(5, 0),
        },
        Weapon,
        WeaponTimer(Stopwatch::new()),
    ));

    next_state.set(GameState::Gaming);
}

fn decorate_world(mut commands: Commands, texture_handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::rng();

    for _ in 0..NUM_DECORATIONS {
        let x = rng.random_range(-WORLD_WIDTH..WORLD_WIDTH);
        let y = rng.random_range(-WORLD_HEIGHT..WORLD_HEIGHT);
        commands.spawn((
            SpriteBundle {
                texture: texture_handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: texture_handle.layout.clone().unwrap(),
                index: get_sprite_index(7, rng.random_range(0..8)),
            },
        ));
    }
}
