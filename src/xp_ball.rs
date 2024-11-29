use bevy::prelude::*;
use crate::config::CONFIG;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use crate::util::get_sprite_index;

#[derive(Component, Default)]
pub struct XPBall;

impl XPBall {
    pub fn spawn(commands: &mut Commands, translation: Vec3, texture_handle: &Res<GlobalTextureAtlas>) {
        commands.spawn((
            SpriteBundle {
                texture: texture_handle.image.clone().unwrap(),
                transform: Transform::from_translation(translation).with_scale(Vec3::splat(CONFIG.sprite.sprite_scale_factor)),
                ..default()
            },
            TextureAtlas {
                layout: texture_handle.layout.clone().unwrap(),
                index: get_sprite_index(5, 2),
            },
            XPBall::default(),
        ));
    }
}

pub struct XPBallPlugin;

impl Plugin for XPBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_pickup_xp.run_if(in_state(GameState::Gaming))
        );
    }
}

fn handle_player_pickup_xp(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player), With<Player>>,
    mut xp_ball_query: Query<(&Transform, Entity), With<XPBall>>,
) {
    for (player_transform, mut player) in player_query.iter_mut() {
        for (xp_ball_transform, xp_ball_entity) in xp_ball_query.iter_mut() {
            if player_transform.translation.distance_squared(xp_ball_transform.translation) <= 1500.0 {
                player.xp += 1;
                commands.entity(xp_ball_entity).despawn();
            }
        }
    }
}