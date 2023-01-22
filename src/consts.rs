use bevy::{math::vec2, prelude::Vec2};

// Player
pub const PLAYER_SIZE: Vec2 = vec2(100., 100.);
pub const PLAYER_ANIMATION_FPS: f32 = 12.;
pub const PLAYER_SPEED: f32 = 300.;
pub const ARROW_DISTANCE: f32 = 100.;
pub const ARROW_SIZE: Vec2 = vec2(40., 0.); // The default arrow size (As of now, the Y value is not used)
pub const SLIME_THROW_MULTIPLIER: f32 = 5000.; // How fast the throw speed grows as the mouse button is held
pub const SLIME_THROW_MINIMUM_VALUE: f32 = 300.; // Minimum throw speed
pub const ARROW_SIZE_MULTIPLIER: f32 = 0.01; // Controls how fast the arrow grows
pub const SLIME_THROW_MULTIPLIER_VALUE: f32 = 10000.; // Maximum throw speed
pub const PLAYER_LAYER: f32 = 9.;

// Slime
pub const SLIME_RELATIVE_SIZE: f32 = 0.5; // Slimes' size relative to the player's
                                          // const SLIME_POSITION_UPDATE_FREQUENCY: f32 = 0.1;
pub const SLIME_ANIMATION_FPS: f32 = 12.;
pub const SLIME_FOLLOW_WEIGHT: f32 = 0.04; // The lower this number is, the smoother the slime following becomes, but the slower the slimes get
pub const SLIME_PADDING: f32 = 20.; // Slimes will stop moving if they're at this distance from the slime in front of them
pub const SLIME_LAYER: f32 = 1.;

// Tilemaps
// pub const TILE1: String = String::from("=");
// pub const TILE2: String = String::from(".");
// pub const WALL1: String = String::from("#");
//pub const TILE1: String = String::from("#");
