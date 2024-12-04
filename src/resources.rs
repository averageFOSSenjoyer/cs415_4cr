use crate::config::CONFIG;
use crate::state::GameState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb_u8(
            CONFIG.game.background_color.0,
            CONFIG.game.background_color.1,
            CONFIG.game.background_color.2,
        )))
        .insert_resource(GlobalTextureAtlas::default())
        .insert_resource(CursorPosition(None))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            update_cursor_position.run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub(crate) layout: Option<Handle<TextureAtlasLayout>>,
    pub(crate) image: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct CursorPosition(pub(crate) Option<Vec2>);

fn load_assets(
    mut texture_handle: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    texture_handle.image = Some(asset_server.load(CONFIG.sprite.spritesheet_path.clone()));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(CONFIG.sprite.sprite_height, CONFIG.sprite.sprite_width),
        CONFIG.sprite.spritesheet_width,
        CONFIG.sprite.spritesheet_height,
        None,
        None,
    );
    texture_handle.layout = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::Initializing);
}

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_position.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_position.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());
}
