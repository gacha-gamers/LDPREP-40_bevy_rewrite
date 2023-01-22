use crate::global::{Handles, HandlesProgress};
use crate::GameStates;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameStates::LoadingScreen)
                .continue_to_state(GameStates::Game)
                .with_collection::<Handles>(),
        );
    }
}
