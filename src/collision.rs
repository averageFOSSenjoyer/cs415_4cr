use crate::config::CONFIG;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::state::GameState;
use crate::weapon::Projectile;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_enemy_projectile_collision,
                handle_player_enemy_collision,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}

fn handle_enemy_projectile_collision(
    mut commands: Commands,
    projectile_query: Query<(&Transform, Entity), With<Projectile>>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
) {
    if projectile_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for (projectile_transform, projectile_entity) in projectile_query.iter() {
        for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if projectile_transform
                .translation
                .distance_squared(enemy_transform.translation)
                <= 250.0
            {
                enemy.health -= CONFIG.player.projectile_damage;
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}

fn handle_player_enemy_collision(
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let (player_transform, mut player) = player_query.single_mut();

    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        if player_transform
            .translation
            .distance_squared(enemy_transform.translation)
            <= 250.0
            && enemy.attack_timer.elapsed_secs() > 1.0
        {
            player.health -= 0.25;
            enemy.attack_timer.reset();
        }
    }
}
