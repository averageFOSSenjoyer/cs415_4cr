use crate::player::Player;
use crate::state::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::math::vec3;
use bevy::prelude::{
    in_state, Camera, Camera2dBundle, Commands, IntoSystemConfigs, OnEnter, Query, Transform, With,
    Without,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), init_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::Gaming)),
            );
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if player_query.is_empty() || camera_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;
    let (x, y) = (player_transform.x, player_transform.y);

    camera_transform.translation = vec3(x, y, 0.0);
}
