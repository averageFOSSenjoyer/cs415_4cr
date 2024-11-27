use crate::player::Player;
use crate::resources::{CursorPosition, GlobalTextureAtlas};
use crate::state::GameState;
use crate::util::get_sprite_index;
use crate::{ATTACK_INTERVAL, PROJECTILE_SPEED, SPRITE_SCALE_FACTOR};
use bevy::app::{App, Plugin};
use bevy::math::{vec2, vec3, Quat, Vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Weapon;
#[derive(Component)]
pub struct WeaponTimer(pub Stopwatch);
#[derive(Component)]
pub struct Projectile;
#[derive(Component)]
pub struct ProjectileDirection(Vec3);

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_weapon_transform,
                handle_weapon_input,
                update_projectiles,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}

fn update_weapon_transform(
    cursor_position: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut weapon_query: Query<&mut Transform, (With<Weapon>, Without<Player>)>,
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation.truncate();
    let mut weapon_transform = weapon_query.single_mut();
    let cursor_position = cursor_position.0.unwrap_or(player_position);

    let angle =
        (player_position.y - cursor_position.y).atan2(player_position.x - cursor_position.x) + PI;

    // mirrors weapon if it's on the other side
    if PI / 2.0 < angle && angle < 3.0 * PI / 2.0 {
        weapon_transform.rotation = Quat::from_rotation_z(angle + PI);
    } else {
        weapon_transform.rotation = Quat::from_rotation_z(angle);
    }

    let offset = 25.0;
    let new_weapon_pos = vec2(
        player_position.x + offset * angle.cos(),
        player_position.y + offset * angle.sin(),
    );

    weapon_transform.translation = vec3(new_weapon_pos.x, new_weapon_pos.y, 10.0);
}

fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    mut weapon_query: Query<(&Transform, &mut WeaponTimer), With<Weapon>>,
    player_query: Query<&Transform, With<Player>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    texture_handle: Res<GlobalTextureAtlas>,
) {
    if weapon_query.is_empty() || player_query.is_empty() {
        return;
    }

    let (weapon_transform, mut weapon_timer) = weapon_query.single_mut();
    let weapon_position = weapon_transform.translation.truncate();
    weapon_timer.0.tick(time.delta());

    if !mouse_input.pressed(MouseButton::Left) || weapon_timer.0.elapsed_secs() < ATTACK_INTERVAL {
        return;
    }

    let player_position = player_query.single().translation;

    // due to mirroring, we need to which side the weapon is on
    let projectile_direction = if weapon_transform.translation.x - player_position.x < 0.0 {
        -weapon_transform.local_x()
    } else {
        weapon_transform.local_x()
    };

    weapon_timer.0.reset();
    commands.spawn((
        SpriteBundle {
            texture: texture_handle.image.clone().unwrap(),
            transform: Transform::from_translation(vec3(weapon_position.x, weapon_position.y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_handle.layout.clone().unwrap(),
            index: get_sprite_index(5, 1),
        },
        Projectile,
        ProjectileDirection(*projectile_direction),
    ));
}

fn update_projectiles(
    mut projectile_query: Query<(&mut Transform, &ProjectileDirection), With<Projectile>>,
) {
    if projectile_query.is_empty() {
        return;
    }

    for (mut transform, direction) in projectile_query.iter_mut() {
        transform.translation += direction.0.normalize_or_zero() * Vec3::splat(PROJECTILE_SPEED);
    }
}
