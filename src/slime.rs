use bevy::{prelude::*, utils::HashMap};
use crate::{GameStates, player::{Player, PLAYER_SIZE, SlimeThrowEvent}, global::{Handles, AnimatedSprite, Moves, GameLayer}};

const SLIME_RELATIVE_SIZE: f32 = 0.5; // Slimes' size relative to the player's
// const SLIME_POSITION_UPDATE_FREQUENCY: f32 = 0.1; 
const SLIME_ANIMATION_FPS: f32 = 12.; 
const SLIME_FOLLOW_WEIGHT: f32 = 0.04; // The lower this number is, the smoother the slime following becomes, but the slower the slimes get
const SLIME_PADDING: f32 = 20.; // Slimes will stop moving if they're at this distance from the slime in front of them
const SLIME_LAYER: f32 = 1.;

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameStates::Game).with_system(slime_spawner).with_system(spawn_slime).with_system(slime_follow).with_system(slime_copy_player_animation).with_system(slime_throw)
        ).add_event::<SpawnSlimeEvent>();
    }
}

pub struct SpawnSlimeEvent;

#[derive(Component)]
pub struct Slime {
    pub following_player: bool,
}

fn slime_spawner(
    player_query: Query<&Transform, With<Player>>,
    assets: Res<Handles>,
    mut event: EventReader<SpawnSlimeEvent>,
    mut commands: Commands,
) {
    for _ in event.iter() {
        let player_tl = player_query.get_single().unwrap().translation;
        

        commands.spawn(
        (
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(PLAYER_SIZE * SLIME_RELATIVE_SIZE), 
                    ..Default::default()
                },
                texture_atlas: assets.player_forward.clone(),
                transform: Transform {
                    translation: player_tl,
                    ..Default::default()
                },
                
                ..Default::default()
            },
            AnimatedSprite { timer: Timer::from_seconds(1. / SLIME_ANIMATION_FPS, TimerMode::Repeating)},
            Moves::default(),
            Slime { following_player: true},
            GameLayer(SLIME_LAYER),
        ));
    }
}

fn spawn_slime (
    input: Res<Input<KeyCode>>,
    mut event: EventWriter<SpawnSlimeEvent>,
) {
    if input.any_just_pressed([KeyCode::I]) {
        event.send(SpawnSlimeEvent)
    }
}

fn slime_follow (
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut slime_query: Query<(Entity, &mut Transform, &Slime), Without<Player>>,
    mut positions_hash_map: Local<HashMap<Entity, Vec3>>,
    mut follow_order: Local<Vec<Entity>>,
) {
    let (player, player_tf)= player_query.get_single().unwrap();
    
    // If it is the first run, add the player position to the dictionary and to the vector
    if !positions_hash_map.contains_key(&player) {
        positions_hash_map.insert(player, player_tf.translation);
    }

    if follow_order.is_empty() {
        follow_order.push(player);
    }
    
    // Update the positions 
    positions_hash_map.insert(player, player_tf.translation);

    slime_query.iter().for_each(|(slime,slime_tf, _)| {
        positions_hash_map.insert(slime, slime_tf.translation);
    });

    for (slime_entity, mut slime_tf, slime) in slime_query.iter_mut() {
    
        // If the slime is not following the player, don't do anything
        if !slime.following_player {
            if follow_order.contains(&slime_entity) {
                follow_order.retain(|x| *x != slime_entity);
                positions_hash_map.remove(&slime_entity);
            }
            continue;
        }

        // If the follow order does not contain this entity, append it
        if !follow_order.contains(&slime_entity) {
            follow_order.push(slime_entity);
        }

        // Find the value of the previous entity in line
        let previous_entity = follow_order[follow_order.iter().position(|x| *x == slime_entity).unwrap() - 1];

        let position_to_lerp = positions_hash_map.get(&previous_entity).unwrap();

        if (slime_tf.translation - *position_to_lerp).length() < SLIME_PADDING {continue;}

        slime_tf.translation = slime_tf.translation + (*position_to_lerp - slime_tf.translation) * SLIME_FOLLOW_WEIGHT;

    }
}

fn slime_copy_player_animation (
    player_query: Query<(&Handle<TextureAtlas>, &TextureAtlasSprite), (Or<(Changed<Handle<TextureAtlas>>, Changed<TextureAtlasSprite>)>, With<Player>, Without<Slime>)>,
    mut slime_query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite), (With<Slime>, Without<Player>)>
) {
    let (player_handle, player_sprite)= if let Ok(player) = player_query.get_single() {
        player
    } else {return;};

    for (mut slime_handle, mut slime_sprite) in slime_query.iter_mut() {
        *slime_handle = player_handle.clone();
        slime_sprite.flip_x = player_sprite.flip_x;
    }
}

fn slime_throw(
    mut slime_query: Query<(&mut Moves, &mut Slime)>,
    mut event: EventReader<SlimeThrowEvent>,
) {
    //TODO! put slime in the same position as the player right before it's thrown
    for throw in event.iter() {
        for (mut slime_moves, mut slime) in slime_query.iter_mut() {
           if !slime.following_player {
               continue; 
           }

           slime_moves.speed = throw.speed;
           slime_moves.direction = throw.direction;
           slime.following_player = false;

           break
        }
    }
}