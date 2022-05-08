use crate::game::Team;
use crate::messages::{ConnectionBroadcast, DisconnectBroadcast, RejectReason, StartGame};
use crate::{Connection, GameIp, GameState, MultiplayerType, Name, Response};
use bevy::prelude::PositionType::Absolute;
use bevy::prelude::*;
use carrier_pigeon::net::{CIdSpec, Config};
use carrier_pigeon::{CId, Client, MsgTableParts, OptionPendingClient, Server};
use std::f32::consts::PI;

pub struct LobbyPlugin;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// All lobby items have this so that they can be cleaned up easily.
struct LobbyItem;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// A marker for the status indicator text field.
struct StatusLabel;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// The player marker
enum Player {
    One,
    Two,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum LobbyButton {
    Back,
    Start,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Players {
    pub p1: Option<(CId, String)>,
    pub p2: Option<(CId, String)>,
    pub me: Option<Team>,
}

impl Players {
    fn first(&self) -> Option<(CId, String)> {
        if self.p1.is_some() {
            self.p1.clone()
        } else {
            self.p2.clone()
        }
    }

    fn count(&self) -> usize {
        match (&self.p1, &self.p2) {
            (None, None) => 0,
            (Some(_), None) => 1,
            (None, Some(_)) => 1,
            (Some(_), Some(_)) => 2,
        }
    }

    fn remove_cid(&mut self, cid: CId) -> bool {
        if let Some((c, s)) = &self.p1 {
            if *c == cid {
                println!("Removing {}", s);
                self.p1 = None;
                return true;
            }
        }
        if let Some((c, s)) = &self.p2 {
            if *c == cid {
                println!("Removing {}", s);
                self.p2 = None;
                return true;
            }
        }

        false
    }

    fn add(&mut self, cid: CId, name: String) -> bool {
        println!("Adding player {} with cid  {}", name, cid);
        match (&self.p1, &self.p2) {
            (None, _) => {
                self.p1 = Some((cid, name));
                true
            }
            (Some(_), None) => {
                self.p2 = Some((cid, name));
                true
            }
            (Some(_), Some(_)) => false,
        }
    }
}

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Lobby)
                .with_system(setup_networking)
                .with_system(setup_lobby_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Lobby)
                .with_system(handle_ui)
                .with_system(game_start)
                .with_system(connect_client)
                .with_system(handle_connections)
                .with_system(handle_disconnections)
                .with_system(update_status)
                .with_system(update_player_labels),
        )
        .add_system_set(SystemSet::on_exit(GameState::Lobby).with_system(clean_up));
    }
}

fn setup_networking(
    ip: Res<GameIp>,
    name: Res<Name>,
    mut commands: Commands,
    multiplayer_type: Res<MultiplayerType>,
    parts: Res<MsgTableParts>,
) {
    match *multiplayer_type {
        MultiplayerType::Server => {
            println!("server");
            commands.insert_resource(Server::new(ip.0, parts.clone(), Config::default()).unwrap());
        }
        MultiplayerType::Host => {
            println!("host");
            commands.insert_resource(Server::new(ip.0, parts.clone(), Config::default()).unwrap());
            commands.insert_resource(
                Client::new(ip.0, parts.clone(), Config::default(), Connection::new(name.0.clone())).option(),
            );
        }
        MultiplayerType::Client => {
            println!("client");
            commands.insert_resource(
                Client::new(ip.0, parts.clone(), Config::default(), Connection::new(name.0.clone())).option(),
            );
        }
    }
}

fn connect_client(
    pending: Option<ResMut<OptionPendingClient>>,
    mut players: ResMut<Players>,
    mut commands: Commands,
) {
    if let Some(mut pending) = pending {
        if pending.done().unwrap() {
            if let Ok((client, resp)) = pending.take::<Response>().unwrap() {
                println!("Client Connected!");
                if let Response::Accepted(_this_cid, optional_player) = resp {
                    if let Some((p_cid, p)) = optional_player {
                        players.add(p_cid, p);
                        players.me = Some(Team::Right);
                    } else {
                        players.me = Some(Team::Left);
                    }
                }
                commands.insert_resource(client);
            }
            commands.remove_resource::<OptionPendingClient>()
        }
    }
}

fn setup_lobby_ui(mut commands: Commands, assets: Res<AssetServer>) {
    println!("Setting up lobby");

    commands.insert_resource(Players::default());

    let font = assets.load("FiraMono-Medium.ttf");
    let arrow = assets.load("arrow.png");
    let text_style = TextStyle {
        font,
        color: Color::BLACK,
        font_size: 60.0,
    };
    let button_style = Style {
        size: Size::new(Val::Px(64.0), Val::Px(64.0)),
        // margin: Rect::all(Val::Px(20.0)),
        margin: Rect {
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            bottom: Val::Auto,
            right: Val::Auto,
        },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    // back button
    commands
        .spawn_bundle(ButtonBundle {
            color: UiColor(Color::WHITE),
            style: button_style.clone(),
            image: arrow.clone().into(),
            ..Default::default()
        })
        .insert(LobbyItem)
        .insert(LobbyButton::Back);

    // parent
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: Absolute,
                margin: Rect::all(Val::Auto),
                padding: Rect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                },
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(LobbyItem)
        .with_children(|parent| {
            // Title
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        bottom: Val::Px(0.0),
                        ..Rect::all(Val::Px(20.0))
                    },
                    ..Default::default()
                },
                text: Text::with_section("Lobby", text_style.clone(), TextAlignment::default()),
                ..Default::default()
            });

            // Status
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Status: --",
                        TextStyle {
                            font_size: 40.0,
                            ..text_style.clone()
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(StatusLabel);

            // Players holder
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: Rect::all(Val::Auto),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Player 1",
                                text_style.clone(),
                                TextAlignment::default(),
                            ),
                            style: Style {
                                margin: Rect::all(Val::Px(40.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Player::One);

                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "Player 2",
                                text_style.clone(),
                                TextAlignment::default(),
                            ),
                            style: Style {
                                margin: Rect::all(Val::Px(40.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Player::Two);
                });

            // Start Arrow
            parent
                .spawn_bundle(ButtonBundle {
                    color: UiColor(Color::WHITE),
                    style: Style {
                        margin: Rect {
                            top: Val::Px(10.0),
                            left: Val::Auto,
                            bottom: Val::Auto,
                            right: Val::Px(10.0),
                        },
                        ..button_style.clone()
                    },
                    transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
                    image: arrow.into(),
                    ..Default::default()
                })
                .insert(LobbyItem)
                .insert(LobbyButton::Start);
        });
}

fn game_start(client: Option<Res<Client>>, mut game_state: ResMut<State<GameState>>) {
    if let Some(client) = client {
        if client.recv::<StartGame>().count() >= 1 {
            let _ = game_state.set(GameState::Game);
        }
    }
}

fn update_status(
    mut q_status: Query<&mut Text, With<StatusLabel>>,
    multiplayer_type: Res<MultiplayerType>,
    client: Option<Res<Client>>,
    server: Option<Res<Server>>,
) {
    let status = format!(
        "Status: {}",
        match *multiplayer_type {
            MultiplayerType::Client => {
                match client {
                    Some(client) if client.open() => "Client connected",
                    _ => "Client not connected",
                }
            }
            _ => {
                if server.is_some() {
                    "Server Listening"
                } else {
                    "No Server"
                }
            }
        }
    );

    for mut c_status in q_status.iter_mut() {
        c_status.sections[0].value = status.clone();
    }
}

fn update_player_labels(mut q_player_label: Query<(&mut Text, &Player)>, players: Res<Players>) {
    let p1_txt = match &players.p1 {
        None => "Player 1".to_owned(),
        Some(p) => p.1.clone(),
    };
    let p2_txt = match &players.p2 {
        None => "Player 2".to_owned(),
        Some(p) => p.1.clone(),
    };

    for (mut text, player) in q_player_label.iter_mut() {
        match player {
            Player::One => text.sections[0].value = p1_txt.clone(),
            Player::Two => text.sections[0].value = p2_txt.clone(),
        }
    }
}

fn handle_ui(
    q_interaction: Query<(&Interaction, &LobbyButton), Changed<Interaction>>,
    mut game_state: ResMut<State<GameState>>,
    players: Res<Players>,
    mut server: Option<ResMut<Server>>,
) {
    for (interaction, button) in q_interaction.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                LobbyButton::Back => {
                    let _ = game_state.set(GameState::Menu);
                }
                LobbyButton::Start => {
                    if let Some(server) = &mut server {
                        if players.count() == 2 {
                            let _ = game_state.set(GameState::Game);
                            server.send_spec(CIdSpec::All, &StartGame).unwrap();
                        }
                    }
                }
            }
        }
    }
}

fn handle_connections(
    server: Option<ResMut<Server>>,
    client: Option<Res<Client>>,
    mut players: ResMut<Players>,
) {
    if let Some(mut server) = server {
        let mut broadcasts = vec![];
        server.handle_new_cons::<Connection, Response>(&mut |cid, c| {
            let existing_player = players.first();
            if players.add(cid, c.name.clone()) {
                println!("Adding new Player");
                broadcasts.push(ConnectionBroadcast::new(c.name, cid));
                (true, Response::Accepted(cid, existing_player))
            } else {
                println!("Rejecting new Player");
                (false, Response::Rejected(RejectReason::MaxPlayersReached))
            }
        });
        for bm in broadcasts {
            println!("Broadcasting");
            server.broadcast(&bm).unwrap();
        }
    } else if let Some(client) = client {
        for msg in client.recv::<ConnectionBroadcast>() {
            players.add(msg.cid, msg.name.clone());
        }
    }
}

fn handle_disconnections(
    server: Option<ResMut<Server>>,
    client: Option<Res<Client>>,
    mut players: ResMut<Players>,
) {
    if let Some(mut server) = server {
        let mut broadcasts = vec![];
        server.handle_disconnects(&mut |cid, _status| {
            broadcasts.push(DisconnectBroadcast { cid });
            players.remove_cid(cid);
        });
        for bm in broadcasts {
            println!("Broadcasting");
            server.broadcast(&bm).unwrap();
        }
    } else if let Some(client) = client {
        for msg in client.recv::<DisconnectBroadcast>() {
            println!("Disconnection broadcast received.");
            players.remove_cid(msg.cid);
        }
    }
}

fn clean_up(mut commands: Commands, q_menu: Query<Entity, With<LobbyItem>>) {
    for e in q_menu.iter() {
        commands.entity(e).despawn_recursive();
    }
}
