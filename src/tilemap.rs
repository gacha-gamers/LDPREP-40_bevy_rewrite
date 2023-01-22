use bevy::prelude::*;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Resource)]
struct Tiles {}
fn load_tiles() {}
