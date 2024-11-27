use crate::enemy::Enemy;
use crate::state::GameState;
use crate::weapon::Projectile;
use crate::PROJECTILE_DAMAGE;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_enemy_projectile_collision.run_if(in_state(GameState::Gaming)),
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
                enemy.health -= PROJECTILE_DAMAGE;
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}
