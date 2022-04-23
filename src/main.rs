mod plugin;
mod messages;
mod game;
mod menu;
mod lobby;

use bevy::prelude::*;
use crate::messages::{Connection, Disconnect, Response};
use crate::plugin::{AppExt, ClientPlugin, ServerPlugin};
use bevy::render::camera::ScalingMode;
use bevy_editor_pls::EditorPlugin;
use heron::prelude::*;
use crate::game::GamePlugin;
use crate::lobby::LobbyPlugin;
use crate::menu::MenuPlugin;

pub type Client = carrier_pigeon::Client<Connection, Response, Disconnect>;
pub type OptionPendingClient = carrier_pigeon::OptionPendingClient<Connection, Response, Disconnect>;
pub type Server = carrier_pigeon::Server<Connection, Response, Disconnect>;
pub type MsgTableParts = carrier_pigeon::MsgTableParts<Connection, Response, Disconnect>;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MultiplayerType {
    Server,
    Host,
    Client,
}

impl MultiplayerType {
    /// Whether this is a server type (`Server` or `Host`).
    pub fn is_server(&self) -> bool {
        match self {
            MultiplayerType::Server => true,
            MultiplayerType::Host   => true,
            MultiplayerType::Client => false,
        }
    }

    /// Whether this is a client type (`Client` or `Host`).
    pub fn is_client(&self) -> bool {
        match self {
            MultiplayerType::Server => false,
            MultiplayerType::Host   => true,
            MultiplayerType::Client => true,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Menu.
    Menu,
    /// Client connecting/Server waiting for client.
    Lobby,
    /// Playing or viewing game.
    Game,
    /// Game is over.
    GameOver,
}

fn main() {
    let mut table = messages::get_table();

    let mut app = App::new();
    app
        .register_net_comp::<Transform>(&mut table)
        .register_net_comp::<Velocity>(&mut table)
    ;

    let parts = table.build::<Connection, Response, Disconnect>().unwrap();

    app
        .insert_resource(parts)

        .insert_resource(WindowDescriptor {
            title: "Bong".into(),
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ClientPlugin::<Connection, Response, Disconnect>::default())
        .add_plugin(ServerPlugin::<Connection, Response, Disconnect>::default())
        .add_plugin(GamePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(LobbyPlugin)

        .add_startup_system(setup)
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