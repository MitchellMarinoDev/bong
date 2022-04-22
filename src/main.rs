mod plugin;
mod messages;
mod game;
mod menu;

use bevy::prelude::*;
use crate::messages::{Connection, Disconnect, Response};
use crate::plugin::ClientPlugin;
use bevy::render::camera::ScalingMode;
use heron::prelude::*;
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;

type Client = carrier_pigeon::Client<Connection, Response, Disconnect>;
type Server = carrier_pigeon::Server<Connection, Response, Disconnect>;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MultiplayerType {
    Server,
    Host,
    Client,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Menu.
    Menu,
    /// Client connecting/Server waiting for client.
    Lobby(MultiplayerType),
    /// Playing or viewing game.
    Game,
    /// Game is over.
    GameOver,
}

fn main() {
    App::new()
        // Plugins
        .insert_resource(WindowDescriptor {
            title: "Bong".into(),
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })

        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ClientPlugin::<Connection, Response, Disconnect>::default())
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)

        .add_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.left = -1920.0/2.0;
    camera.orthographic_projection.right = 1920.0/2.0;
    camera.orthographic_projection.bottom = -1080.0/2.0;
    camera.orthographic_projection.top = 1080.0/2.0;
    commands.spawn_bundle(camera);

    // UI Camera
    commands.spawn_bundle(UiCameraBundle::default());
}