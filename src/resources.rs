use crate::state::GameState;
use crate::{
    BACKGROUND_COLOR, SPRITESHEET_HEIGHT, SPRITESHEET_PATH, SPRITESHEET_WIDTH, SPRITE_HEIGHT,
    SPRITE_WIDTH,
};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::srgb_u8(
                BACKGROUND_COLOR.0,
                BACKGROUND_COLOR.1,
                BACKGROUND_COLOR.2,
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
    texture_handle.image = Some(asset_server.load(SPRITESHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_HEIGHT, SPRITE_WIDTH),
        SPRITESHEET_WIDTH,
        SPRITESHEET_HEIGHT,
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
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}
