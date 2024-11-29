use bevy::prelude::*;
use cs415_project::animation::AnimationPlugin;
use cs415_project::camera::CameraPlugin;
use cs415_project::collision::CollisionPlugin;
use cs415_project::config::CONFIG;
use cs415_project::enemy::EnemyPlugin;
use cs415_project::player::PlayerPlugin;
use cs415_project::resources::ResourcesPlugin;
use cs415_project::state::GameState;
use cs415_project::weapon::WeaponPlugin;
use cs415_project::world::WorldPlugin;
use cs415_project::xp_ball::XPBallPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (CONFIG.app.window_height, CONFIG.app.window_width).into(),
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
        .add_plugins(XPBallPlugin)
        .init_state::<GameState>()
        .run();
}
