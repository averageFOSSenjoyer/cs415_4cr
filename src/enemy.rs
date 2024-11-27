use crate::animation::AnimationTimer;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::util::get_sprite_index;
use crate::{
    ANIMATION_TICK_DURATION, ENEMY_HEALTH, ENEMY_SPAWN_INTERVAL, ENEMY_SPEED, MAX_NUM_ENEMIES,
    SPRITE_SCALE_FACTOR, WORLD_HEIGHT, WORLD_WIDTH,
};
use bevy::app::{App, Plugin};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use std::time::Duration;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_enemies
                .run_if(in_state(GameState::Gaming))
                .run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
        )
        .add_systems(
            Update,
            (update_enemy_transform, despawn_dead_enemy).run_if(in_state(GameState::Gaming)),
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    texture_handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemies_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(5);

    if num_enemies >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let mut rng = rand::rng();
    for _ in 0..enemies_spawn_count {
        let x = rng.random_range(-WORLD_WIDTH..WORLD_WIDTH);
        let y = rng.random_range(-WORLD_HEIGHT..WORLD_HEIGHT);
        commands.spawn((
            SpriteBundle {
                texture: texture_handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: texture_handle.layout.clone().unwrap(),
                index: get_sprite_index(3, 0),
            },
            Enemy::default(),
            AnimationTimer(Timer::from_seconds(
                ANIMATION_TICK_DURATION,
                TimerMode::Repeating,
            )),
        ));
    }
}

fn update_enemy_transform(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if enemy_query.is_empty() || player_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation;
    for mut transform in enemy_query.iter_mut() {
        let direction = (player_position - transform.translation).normalize();
        transform.translation += direction * ENEMY_SPEED;
    }
}

fn despawn_dead_enemy(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity), With<Enemy>>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
            for mut player in player_query.iter_mut() {
                player.xp += 1;
            }
        }
    }
}