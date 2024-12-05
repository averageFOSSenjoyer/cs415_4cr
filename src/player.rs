use crate::animation::AnimationTimer;
use crate::config::CONFIG;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::util::get_sprite_index;
use bevy::app::{App, Plugin, Update};
use bevy::color::palettes::tailwind;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;
use rand::Rng;

#[derive(Component)]
pub struct Player {
    pub xp: u32,
    pub level: u32,
    pub health: f32,
    pub attack_speed_multiplier: f32,
    pub movement_speed_multiplier: f32,
    pub xp_ball_pickup_range_multiplier: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            xp: 0,
            level: 0,
            health: 1.0,
            attack_speed_multiplier: 1.0,
            movement_speed_multiplier: 1.0,
            xp_ball_pickup_range_multiplier: 1.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_player, init_stat_bars.after(init_player)).run_if(in_state(GameState::Initializing)),
        )
        .add_systems(
            Update,
            (handle_player_input, check_player_death, handle_player_xp, update_stat_bars)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}

fn init_player(
    mut commands: Commands,
    texture_handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Player::default(),
        Sprite {
            image: texture_handle.image.clone().unwrap(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_handle.layout.clone().unwrap(),
                index: get_sprite_index(0, 0),
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(CONFIG.sprite.sprite_scale_factor)),
        AnimationTimer(Timer::from_seconds(
            CONFIG.game.animation_tick_interval,
            TimerMode::Repeating,
        )),
    ));

    next_state.set(GameState::Gaming);
}

fn handle_player_input(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, player) = player_query.single_mut();
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
            transform.translation.x
                + player.movement_speed_multiplier
                    * delta.x
                    * CONFIG.player.movement_speed
                    * time.delta_secs(),
            -CONFIG.game.world_width,
        )
    } else {
        f32::min(
            transform.translation.x
                + player.movement_speed_multiplier
                    * delta.x
                    * CONFIG.player.movement_speed
                    * time.delta_secs(),
            CONFIG.game.world_width,
        )
    };
    transform.translation.y = if delta.y < 0.0 {
        f32::max(
            transform.translation.y
                + player.movement_speed_multiplier
                    * delta.y
                    * CONFIG.player.movement_speed
                    * time.delta_secs(),
            -CONFIG.game.world_height,
        )
    } else {
        f32::min(
            transform.translation.y
                + player.movement_speed_multiplier
                    * delta.y
                    * CONFIG.player.movement_speed
                    * time.delta_secs(),
            CONFIG.game.world_height,
        )
    };

    transform.translation.z = 10.0; // keep player above
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

fn handle_player_xp(mut player_query: Query<&mut Player, With<Player>>) {
    for mut player in player_query.iter_mut() {
        if player.xp >= 5 + player.level * 3 {
            player.level += 1;
            player.xp = 0;
            player.health = (player.health + CONFIG.player.health_per_lvlup).min(1.0);
            let mut rng = rand::rng();
            let num = rng.random_range(0..3);
            if num == 0 {
                player.movement_speed_multiplier += CONFIG.player.movement_speed_multiplier_inc;
            } else if num == 1 {
                player.attack_speed_multiplier += CONFIG.player.attack_speed_multiplier_inc;
            } else {
                player.xp_ball_pickup_range_multiplier +=
                    CONFIG.player.xp_ball_pickup_range_multiplier_inc;
            }
        }
    }
}

#[derive(Component, Default)]
pub struct HealthBar;

#[derive(Component, Default)]
pub struct XPBar;

fn init_stat_bars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<Entity, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_entity = player_query.single_mut();

    commands.entity(player_entity).with_children(|children| {
        children.spawn((
           HealthBar::default(),
           Mesh2d {
               0: meshes
                   .add(Rectangle::new(40.0, 5.0))
                   .into(),
           },
           MeshMaterial2d {
               0: materials.add(ColorMaterial::from_color(tailwind::RED_500)),
           },
           Transform::from_translation(Vec3::new(0., 28.0, 101.0)),
        ));

        children.spawn((
            Mesh2d {
                0: meshes
                    .add(Rectangle::new(40.0, 5.0))
                    .into(),
            },
            MeshMaterial2d {
                0: materials.add(ColorMaterial::from_color(tailwind::NEUTRAL_800)),
            },
            Transform::from_translation(Vec3::new(0., 28.0, 100.0)),
        ));

        children.spawn((
            XPBar::default(),
            Mesh2d {
                0: meshes
                    .add(Rectangle::new(40.0, 5.0))
                    .into(),
            },
            MeshMaterial2d {
                0: materials.add(ColorMaterial::from_color(tailwind::BLUE_600)),
            },
            Transform::from_translation(Vec3::new(0., 22.0, 101.0)),
        ));

        children.spawn((
            Mesh2d {
                0: meshes
                    .add(Rectangle::new(40.0, 5.0))
                    .into(),
            },
            MeshMaterial2d {
                0: materials.add(ColorMaterial::from_color(tailwind::NEUTRAL_800)),
            },
            Transform::from_translation(Vec3::new(0., 22.0, 100.0)),
        ));
    });
}

fn update_stat_bars(
    player_query: Query<&Player, With<Player>>,
    mut health_bar_query: Query<&mut Transform, (With<HealthBar>, Without<XPBar>)>,
    mut xp_bar_query: Query<&mut Transform, (With<XPBar>, Without<HealthBar>)>,
) {
    if player_query.is_empty() || health_bar_query.is_empty() || xp_bar_query.is_empty() {
        return;
    }

    let player = player_query.single();

    let player_health = player.health;
    let mut health_bar_transform = health_bar_query.single_mut();
    health_bar_transform.translation.x = (1.0 - player_health) * -20.0;
    health_bar_transform.scale.x = player_health;

    let player_xp_percentage = player.xp as f32 / (5 + player.level * 3) as f32;
    let mut xp_bar_transform = xp_bar_query.single_mut();
    xp_bar_transform.translation.x = (1.0 - player_xp_percentage) * -20.0;
    xp_bar_transform.scale.x = player_xp_percentage;
}
