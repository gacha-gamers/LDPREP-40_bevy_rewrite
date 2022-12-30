mod slime;
mod global;
mod loading_screen;
mod player;

use bevy::prelude::*;
use global::GlobalPlugin;
use loading_screen::LoadingScreenPlugin;
use player::PlayerPlugin;
use slime::SlimePlugin;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum GameStates {
    Game,
    LoadingScreen,
}

#[derive(Component)]
struct BandanaDee;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bandana dee is the greatest!".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }).set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_state(GameStates::LoadingScreen)
        .add_plugin(LoadingScreenPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GlobalPlugin)
        .add_plugin(SlimePlugin)
        .run();
}
