use crate::animation::AnimationTimer;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::util::get_sprite_index;
use crate::{
    ANIMATION_TICK_DURATION, PLAYER_SPEED, SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH,
};
use bevy::app::{App, Plugin, Update};
use bevy::color::palettes::tailwind;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_progressbar::{ProgressBar, ProgressBarBundle, ProgressBarMaterial, ProgressBarPlugin};

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
            (handle_player_input, update_player_health_bar, check_player_death).run_if(in_state(GameState::Gaming)),
        ).add_plugins(
            ProgressBarPlugin
        );
    }
}

fn init_player(
    mut commands: Commands,
    texture_handle: Res<GlobalTextureAtlas>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
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

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Grid,
                left: Val::Percent(47.5),
                right: Val::Percent(47.5),
                top: Val::Percent(45.0),
                bottom: Val::Percent(54.0),
                ..default()
            },
            ..default()
        })
        .with_children(|wrapper| {
            let mut bar = ProgressBar::single(tailwind::RED_500.into());
            bar.set_progress(1.0);
            wrapper.spawn(ProgressBarBundle::new(
                bar,
                &mut materials,
            ));
        });

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

fn update_player_health_bar(
    player_query: Query<&Player, With<Player>>,
    mut player_health_bar_query: Query<&mut ProgressBar, With<ProgressBar>>,
) {
    let player = player_query.single();
    let mut health_bar = player_health_bar_query.single_mut();
    health_bar.set_progress(player.health);
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