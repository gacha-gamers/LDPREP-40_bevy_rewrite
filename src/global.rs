use bevy::prelude::*;

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
   fn build(&self, app: &mut App) {
       app
       .add_system(animate_sprite)
       .add_system(moves_system);
   } 
}

#[derive(Resource)]
pub struct Handles {
    pub player_forward: Handle<TextureAtlas>, 
    pub player_up: Handle<TextureAtlas>, 
    pub player_sideways: Handle<TextureAtlas>, 
}

#[derive(Resource, Default, Debug)]
pub struct HandlesProgress(pub Vec<HandleUntyped>);

#[derive(Component)]
pub struct AnimatedSprite {
    // the value of this timer should be 1 / fps
    pub timer: Timer 
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimatedSprite,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}


#[derive(Component, Default)]
pub struct Moves {
    pub velocity: Vec2,
    pub speed: f32,
}

fn moves_system(
    time: Res<Time>,
    mut move_query: Query<(&Moves, &mut Transform)>,
) {
    for (mover, mut tf) in move_query.iter_mut() {
        tf.translation += mover.velocity.extend(0.) * mover.speed * time.delta_seconds();
    }
}