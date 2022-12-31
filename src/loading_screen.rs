use crate::global::{Handles, HandlesProgress};
use crate::GameStates;
use bevy::asset::LoadState;
use bevy::math::vec2;
use bevy::prelude::*;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStates::LoadingScreen).with_system(handles_load),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::LoadingScreen).with_system(check_assets_ready),
        );
    }
}

fn handles_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut loading = HandlesProgress::default();

    let mut texture_atlas_loader = |path: &str, size: Vec2, columns: usize, rows: usize| {
        let handle = asset_server.load(path);
        loading.0.push(handle.clone_untyped());

        let texture_atlas = TextureAtlas::from_grid(handle, size, columns, rows, None, None);

        return texture_atlases.add(texture_atlas);
    };

    let player_forward = 
    texture_atlas_loader("mini_slime_forward.png", vec2(27., 20.), 5, 1);
    
    let player_up=
    texture_atlas_loader("mini_slime_up.png", vec2(24., 18.), 5, 1);
    
    let player_sideways =
    texture_atlas_loader("mini_slime-Sheet.png", vec2(22., 20.), 5, 1);

    let pointing_arrow: Handle<Image> = asset_server.load("arrow.png");
    loading.0.push(pointing_arrow.clone_untyped());

    commands.insert_resource(loading);
    commands.insert_resource(Handles { player_forward, player_up, player_sideways, pointing_arrow });
}

fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<HandlesProgress>,
    mut app_state: ResMut<State<GameStates>>,
) {
    match server.get_group_load_state(loading.0.iter().map(|h: &HandleUntyped| h.id)) {
        LoadState::Failed => {
            panic!("One of the assets failed to load!");
        }
        LoadState::Loaded => {
            println!("Loaded all resources!");
            commands.remove_resource::<HandlesProgress>();
            app_state.set(GameStates::Game).unwrap();
        }
        LoadState::NotLoaded | LoadState::Loading => {
            println!("Loading...");
        }
        other => {
            println!("What am I doing with my life: {:?}", other);
        }
    }
}
