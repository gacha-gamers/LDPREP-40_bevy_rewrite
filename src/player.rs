use bevy::math::vec2;
use bevy::prelude::*;

use crate::global::{AnimatedSprite, Handles, Moves};
use crate::GameStates;

pub const PLAYER_SIZE: Vec2 = vec2(100., 100.);
const PLAYER_ANIMATION_FPS: f32 = 12.;
const PLAYER_SPEED: f32 = 300.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::Game).with_system(player_startup))
            .add_system_set(
                SystemSet::on_update(GameStates::Game).with_system(player_movement).with_system(slime_animations),
            );
    }
}

fn player_startup(mut commands: Commands, assets: Res<Handles>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.player_forward.clone(),
            sprite: TextureAtlasSprite {
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            ..Default::default()
        },
        AnimatedSprite {
            timer: Timer::from_seconds(1. / PLAYER_ANIMATION_FPS, TimerMode::Repeating),
        },
        Player,
        Moves {
            velocity: Vec2::ZERO,
            speed: PLAYER_SPEED,
        }
    ));
}

#[derive(Component)]
pub struct Player;

fn player_movement(input: Res<Input<KeyCode>>, mut player: Query<(&mut Moves), With<Player>>) {
    let mut velocity = &mut player
        .get_single_mut()
        .expect("No player in game? Please change this line").velocity;
    
    // Reset velocity
    *velocity *= 0.;

    if input.pressed(KeyCode::W) {
        velocity.y += 1.;
    }
    if input.pressed(KeyCode::D) {
        velocity.x += 1.;
    }
    if input.pressed(KeyCode::S) {
        velocity.y += -1.;
    }
    if input.pressed(KeyCode::A) {
        velocity.x += -1.;
    }

    *velocity = velocity.normalize_or_zero();
}

fn slime_animations (
    assets: Res<Handles>,
    mut player_query: Query<(&Moves, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite), With<Player>>,
) {
    let (mover, mut texture, mut sprite) = player_query.get_single_mut().expect("No player in-game? Please change this line");
    let velocity = mover.velocity;

    if velocity.y > 0. {
        *texture = assets.player_up.clone();
    }
    
    if velocity.y < 0. {
        *texture = assets.player_forward.clone();
    }

    if velocity.x > 0. {
        *texture = assets.player_sideways.clone();
        sprite.flip_x = false;
    }

    if velocity.x < 0. {
        *texture = assets.player_sideways.clone();
        sprite.flip_x = true;
    }
}

