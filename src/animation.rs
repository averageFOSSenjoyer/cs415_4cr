use crate::enemy::Enemy;
use crate::player::Player;
use crate::state::GameState;
use crate::util::get_sprite_index;
use bevy::prelude::*;
use crate::config::CONFIG;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animation_timer_tick, animate_player, animate_enemy)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut timer_query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in timer_query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(mut player_query: Query<(&mut TextureAtlas, &mut AnimationTimer), With<Player>>) {
    if player_query.is_empty() {
        return;
    }

    for (mut texture_atlas, timer) in player_query.iter_mut() {
        if timer.just_finished() {
            texture_atlas.index = (texture_atlas.index + 1) % 6;
        }
    }
}

fn animate_enemy(mut enemy_query: Query<(&mut TextureAtlas, &mut AnimationTimer), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut texture_atlas, timer) in enemy_query.iter_mut() {
        if timer.just_finished() {
            texture_atlas.index = get_sprite_index(3, 0)
                + ((texture_atlas.index + 1) % (CONFIG.sprite.spritesheet_width as usize)) % 6;
        }
    }
}
