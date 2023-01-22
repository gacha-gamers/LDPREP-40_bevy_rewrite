use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::AssetCollection;
pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(moves_system)
            .add_system_to_stage(CoreStage::PostUpdate, enforce_layers);
    }
}

#[derive(AssetCollection, Resource, Debug)]
pub struct Handles {
    #[asset(texture_atlas(
        tile_size_x = 27.,
        tile_size_y = 20.,
        columns = 5,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.,
    ))]
    #[asset(path = "mini_slime_forward.png")]
    pub player_forward: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 24.,
        tile_size_y = 18.,
        columns = 5,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.,
    ))]
    #[asset(path = "mini_slime_up.png")]
    pub player_up: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 22.,
        tile_size_y = 20.,
        columns = 5,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.,
    ))]
    #[asset(path = "mini_slime-Sheet.png")]
    pub player_sideways: Handle<TextureAtlas>,

    #[asset(path = "arrow.png")]
    pub pointing_arrow: Handle<Image>,

    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 18.,
        columns = 4,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.,
    ))]
    #[asset(path = "mini_slime_throw-Sheet.png")]
    pub slime_flying: Handle<TextureAtlas>,
    // pub tilemap: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Default, Debug)]
pub struct HandlesProgress(pub Vec<HandleUntyped>);

#[derive(Component)]
pub struct AnimatedSprite {
    // the value of this timer should be 1 / fps
    pub timer: Timer,
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
    pub direction: Vec2,
    pub speed: f32,
}

fn moves_system(time: Res<Time>, mut move_query: Query<(&Moves, &mut Transform)>) {
    for (mover, mut tf) in move_query.iter_mut() {
        tf.translation += mover.direction.extend(0.) * mover.speed * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct GameLayer(pub f32);

// Enforce the layers automatically for ease of use
fn enforce_layers(mut layer_query: Query<(&mut Transform, &GameLayer)>) {
    for (mut tf, layer) in layer_query.iter_mut() {
        tf.translation.z = layer.0;
    }
}
