use std::f32::consts::PI;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::global::{AnimatedSprite, Handles, Moves};
use crate::GameStates;
use crate::slime::Slime;

pub const PLAYER_SIZE: Vec2 = vec2(100., 100.);
const PLAYER_ANIMATION_FPS: f32 = 12.;
const PLAYER_SPEED: f32 = 300.;
const ARROW_DISTANCE: f32 = 100.;
const ARROW_SIZE: Vec2 = vec2(40., 0.); // The default arrow size (As of now, the Y value is not used)
const SLIME_THROW_MULTIPLIER: f32 = 5000.; // How fast the throw speed grows as the mouse button is held
const SLIME_THROW_MINIMUM_VALUE: f32 = 300.; // Minimum throw speed
const ARROW_SIZE_MULTIPLIER: f32 = 0.01; // Controls how fast the arrow grows
const SLIME_THROW_MULTIPLIER_VALUE: f32 = 10000.; // Maximum throw speed
const PLAYER_LAYER: f32 = 9.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStates::Game).with_system(player_startup))
            .add_system_set(
                SystemSet::on_update(GameStates::Game)
                    .with_system(player_movement)
                    .with_system(slime_animations)
                    .with_system(player_aim),
            )
            .add_event::<SlimeThrowEvent>();
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
            transform: Transform {
                translation: Vec2::ZERO.extend(PLAYER_LAYER),
                ..Default::default()
            },
            ..Default::default()
        },
        AnimatedSprite {
            timer: Timer::from_seconds(1. / PLAYER_ANIMATION_FPS, TimerMode::Repeating),
        },
        Player,
        Moves {
            direction: Vec2::ZERO,
            speed: PLAYER_SPEED,
        },
    ));
}

#[derive(Component)]
pub struct Player;

fn player_movement(input: Res<Input<KeyCode>>, mut player: Query<(&mut Moves), With<Player>>) {
    let mut velocity = &mut player
        .get_single_mut()
        .expect("No player in game? Please change this line")
        .direction;

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

fn slime_animations(
    assets: Res<Handles>,
    mut player_query: Query<
        (&Moves, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite),
        With<Player>,
    >,
) {
    let (mover, mut texture, mut sprite) = player_query
        .get_single_mut()
        .expect("No player in-game? Please change this line");
    let velocity = mover.direction;

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

#[derive(Component)]
struct AimLine;

pub struct SlimeThrowEvent {
    pub direction: Vec2,
    pub speed: f32,
}

fn player_aim(
    mouse: Res<Input<MouseButton>>,
    window: Res<Windows>,
    assets: Res<Handles>,
    mut arrow_query: Query<(Entity, &mut Transform, &mut Sprite), (With<AimLine>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<AimLine>)>,
    mut commands: Commands,
    time: Res<Time>,
    mut multiplier: Local<f32>,
    mut slime_throw_event: EventWriter<SlimeThrowEvent>,
    slime_query: Query<&Slime>,
) {
    let window = window.get_primary().unwrap();
    let mouse_position = if let Some(position) = window.cursor_position() {
        position - vec2(window.width(), window.height()) / 2.
    } else {
        return;
    };

    let player_tl = player_query.get_single().unwrap().translation;
    let arrow_direction = (mouse_position - player_tl.truncate()).normalize();

    // Reset the system once the slime is thrown
    if !mouse.pressed(MouseButton::Left) && !arrow_query.is_empty() {
        commands
            .entity(arrow_query.get_single().unwrap().0)
            .despawn();
        
        slime_throw_event.send(
            SlimeThrowEvent {
                direction: arrow_direction,
                speed: *multiplier,
            }
        );
        *multiplier = 0.;
        return;
    }
    
    // If there are no slimes following the player or the button is not pressed, don't do anything
    if slime_query.iter().filter(|x| x.following_player ).collect::<Vec<&Slime>>().is_empty() || !mouse.pressed(MouseButton::Left) {
        return;
    }

    // The arrow gets the player layer
    let arrow_position = player_tl + arrow_direction.extend(0.) * ARROW_DISTANCE;
    // Black magic. Please do not change
    let arrow_angle = Quat::from_rotation_z(arrow_direction.y.atan2(arrow_direction.x) - PI / 2.);

    if let Ok(mut arrow) = arrow_query.get_single_mut() {
        arrow.1.translation = arrow_position;
        arrow.1.rotation = arrow_angle;
        arrow.2.custom_size.insert(vec2(ARROW_SIZE.x, (ARROW_SIZE.y + *multiplier - SLIME_THROW_MINIMUM_VALUE) * ARROW_SIZE_MULTIPLIER));
        if *multiplier < SLIME_THROW_MULTIPLIER_VALUE {
            *multiplier += time.delta_seconds() * SLIME_THROW_MULTIPLIER;
        } 

    } else {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(ARROW_SIZE),
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: arrow_position,
                            rotation: arrow_angle,
                            ..Default::default()
                        },
                        texture: assets.pointing_arrow.clone(),
                        ..Default::default()
                    },
                    AimLine,
                ));
                *multiplier = SLIME_THROW_MINIMUM_VALUE;
            };
}

