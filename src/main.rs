//! Shows how to render simple primitive shapes with a single color.

mod constants;

use bevy::math::{vec2, vec3};
use bevy::time::Stopwatch;
use bevy::{prelude::*, window::*};
use rand::Rng;
use std::f32::consts::PI;

const WINDOW_HEIGHT: f32 = 1200.0;
const WINDOW_WIDTH: f32 = 800.0;
const SPRITE_HEIGHT: f32 = 32.0;
const SPRITE_WIDTH: f32 = 32.0;
const SPRITE_SCALE_FACTOR: f32 = 1.5;
const SPRITESHEET_PATH: &str = "sprites.png";
const SPRITESHEET_HEIGHT: usize = 8;
const SPRITESHEET_WIDTH: usize = 16;
const WORLD_WIDTH: f32 = 10000.0;
const WORLD_HEIGHT: f32 = 10000.0;
const BACKGROUND_COLOR: (u8, u8, u8) = (163, 116, 46);
const DECORATIONS_DENSITY: f32 = 0.0002777777778;
const NUM_DECORATIONS: usize = (WORLD_WIDTH * WORLD_HEIGHT * DECORATIONS_DENSITY) as usize;
const PLAYER_SPEED: f32 = 1.0;
const ATTACK_INTERVAL: f32 = 1.0;
const PROJECTILE_SPEED: f32 = 4.0;

#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);
#[derive(Resource)]
struct CursorPosition(Option<Vec2>);
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Weapon;
#[derive(Component)]
struct WeaponTimer(Stopwatch);
#[derive(Component)]
struct Projectile;
#[derive(Component)]
struct ProjectileDirection(Vec3);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Initializing,
    Gaming,
}

fn main() {
    println!("{:?}", NUM_DECORATIONS);
    App::new()
        .init_state::<GameState>()
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
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb_u8(
            BACKGROUND_COLOR.0,
            BACKGROUND_COLOR.1,
            BACKGROUND_COLOR.2,
        )))
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .insert_resource(CursorPosition(None))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            OnEnter(GameState::Initializing),
            (setup_camera, init_components, decorate_world),
        )
        .add_systems(
            Update,
            (
                handle_player_input,
                update_weapon_transform,
                update_cursor_position,
                handle_weapon_input,
                update_projectiles,
                camera_follow_player,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(Update, close_on_esc)
        .run();
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    image_handle.0 = Some(asset_server.load(SPRITESHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(SPRITE_HEIGHT, SPRITE_WIDTH),
        SPRITESHEET_WIDTH,
        SPRITESHEET_HEIGHT,
        None,
        None,
    );
    texture_atlas.0 = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::Initializing);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn get_sprite_index(row_id: usize, col_id: usize) -> usize {
    assert!(row_id < SPRITESHEET_HEIGHT);
    assert!(col_id < SPRITESHEET_WIDTH);

    row_id * SPRITESHEET_WIDTH + col_id
}

fn init_components(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: get_sprite_index(0, 0),
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Player,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: get_sprite_index(0, 1),
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Weapon,
        WeaponTimer(Stopwatch::new()),
    ));

    next_state.set(GameState::Gaming);
}

fn decorate_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let mut rng = rand::rng();

    for _ in 0..NUM_DECORATIONS {
        let x = rng.random_range(-WORLD_WIDTH..WORLD_WIDTH);
        let y = rng.random_range(-WORLD_HEIGHT..WORLD_HEIGHT);
        commands.spawn((SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: get_sprite_index(7, rng.random_range(0..8)),
            },
            transform: Transform::from_translation(vec3(x, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },));
    }
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

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut();
    let w_key_pressed = keyboard_input.pressed(KeyCode::KeyW);
    let a_key_pressed = keyboard_input.pressed(KeyCode::KeyA);
    let s_key_pressed = keyboard_input.pressed(KeyCode::KeyS);
    let d_key_pressed = keyboard_input.pressed(KeyCode::KeyD);

    let mut delta = Vec2::ZERO;
    if w_key_pressed {
        delta.y += 1.0;
    }
    if s_key_pressed {
        delta.y -= 1.0;
    }
    if a_key_pressed {
        delta.x -= 1.0;
    }
    if d_key_pressed {
        delta.x += 1.0;
    }
    delta = delta.normalize_or_zero();
    transform.translation.x = if delta.x < 0.0 {
        f32::max(transform.translation.x + delta.x * PLAYER_SPEED, -WORLD_WIDTH)
    } else {
        f32::min(transform.translation.x + delta.x * PLAYER_SPEED, WORLD_WIDTH)
    };
    transform.translation.y = if delta.y < 0.0 {
        f32::max(transform.translation.y + delta.y * PLAYER_SPEED, -WORLD_HEIGHT)
    } else {
        f32::min(transform.translation.y + delta.y * PLAYER_SPEED, WORLD_HEIGHT)
    };
    transform.translation.z = 10.0;
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

fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    mut weapon_query: Query<(&Transform, &mut WeaponTimer), With<Weapon>>,
    player_query: Query<&Transform, With<Player>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
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
        SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: get_sprite_index(0, 2),
            },
            transform: Transform::from_translation(vec3(weapon_position.x, weapon_position.y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
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
