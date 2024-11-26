use bevy::{prelude::*, window::*};
use cs415_project::animation::AnimationPlugin;
use cs415_project::camera::CameraPlugin;
use cs415_project::collision::CollisionPlugin;
use cs415_project::enemy::EnemyPlugin;
use cs415_project::player::PlayerPlugin;
use cs415_project::resources::ResourcesPlugin;
use cs415_project::state::GameState;
use cs415_project::weapon::WeaponPlugin;
use cs415_project::world::WorldPlugin;
use cs415_project::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_systems(Update, close_on_esc)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_HEIGHT, WINDOW_WIDTH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AnimationPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(WorldPlugin)
        .run();
}
