use crate::animation::AnimationTimer;
use crate::config::CONFIG;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::util::get_sprite_index;
use crate::xp_ball::XPBall;
use bevy::app::{App, Plugin};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::time::Stopwatch;
use rand::Rng;
use std::time::Duration;

#[derive(Component)]
#[require(Sprite)]
pub struct Enemy {
    pub health: f32,
    pub attack_timer: Stopwatch,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: CONFIG.enemy.enemy_health,
            attack_timer: Stopwatch::new(),
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
                .run_if(on_timer(Duration::from_secs_f32(
                    CONFIG.enemy.enemy_spawn_interval,
                ))),
        )
        .add_systems(
            Update,
            (
                update_enemy_transform,
                despawn_dead_enemy,
                update_enemy_attack_timer,
            )
                .run_if(in_state(GameState::Gaming)),
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
    let enemies_spawn_count =
        (CONFIG.enemy.max_num_enemies - num_enemies).min(CONFIG.enemy.num_per_wave);

    if num_enemies >= CONFIG.enemy.max_num_enemies || player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();

    let mut rng = rand::rng();
    for _ in 0..enemies_spawn_count {
        let mut x = rng.random_range(-CONFIG.game.world_width..CONFIG.game.world_width);
        while (x - player_transform.translation.x).abs() <= CONFIG.app.window_width / 2.0 {
            x = rng.random_range(-CONFIG.game.world_width..CONFIG.game.world_width);
        }
        let mut y = rng.random_range(-CONFIG.game.world_height..CONFIG.game.world_height);
        while (y - player_transform.translation.y).abs() <= CONFIG.app.window_height / 2.0 {
            y = rng.random_range(-CONFIG.game.world_width..CONFIG.game.world_width);
        }
        commands.spawn((
            Enemy::default(),
            Sprite {
                image: texture_handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_handle.layout.clone().unwrap(),
                    index: get_sprite_index(3, 0),
                }),
                ..default()
            },
            Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(CONFIG.sprite.sprite_scale_factor)),
            AnimationTimer(Timer::from_seconds(
                CONFIG.game.animation_tick_interval,
                TimerMode::Repeating,
            )),
        ));
    }
}

fn update_enemy_transform(
    time: Res<Time>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if enemy_query.is_empty() || player_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation;
    for mut transform in enemy_query.iter_mut() {
        let direction = (player_position - transform.translation).normalize();
        transform.translation += direction * CONFIG.enemy.enemy_speed * time.delta_secs()
    }
}

fn despawn_dead_enemy(
    mut commands: Commands,
    enemy_query: Query<(&Transform, &Enemy, Entity), With<Enemy>>,
    texture_handle: Res<GlobalTextureAtlas>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy_transform, enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            XPBall::spawn(&mut commands, enemy_transform.translation, &texture_handle);
            commands.entity(entity).despawn();
        }
    }
}

fn update_enemy_attack_timer(time: Res<Time>, mut enemy_query: Query<&mut Enemy, With<Enemy>>) {
    for mut enemy in enemy_query.iter_mut() {
        enemy.attack_timer.tick(time.delta());
    }
}
