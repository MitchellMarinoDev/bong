use bevy::prelude::*;
use carrier_pigeon::CId;
use crate::{Client, Connection, GameState, MsgTableParts, MultiplayerType, OptionPendingClient, Response, Server};
use crate::messages::{ConnectionBroadcast, RejectReason};

pub struct LobbyPlugin;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// All lobby items have this so that they can be cleaned up easily.
struct LobbyItem;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// A marker for the status indicator text field.
struct StatusLabel;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// The player marker
enum Player {
    One,
    Two,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
struct Players {
    p1: Option<(CId, String)>,
    p2: Option<(CId, String)>,
}

impl Players {
    fn count(&self) -> usize {
        match (&self.p1, &self.p2) {
            (None, None) => 0,
            (Some(_), None) => 1,
            (None, Some(_)) => 1,
            (Some(_), Some(_)) => 2,
        }
    }

    fn add(&mut self, cid: CId, name: String) -> bool {
        match (&self.p1, &self.p2) {
            (None, _) => {
                self.p1 = Some((cid, name));
                true
            },
            (Some(_), None) => {
                self.p2 = Some((cid, name));
                true
            },
            (Some(_), Some(_)) => false,
        }
    }
}

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Players::default())
            .add_system_set(
                SystemSet::on_enter(GameState::Lobby)
                    .with_system(setup_networking)
                    .with_system(setup_lobby_ui)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Lobby)
                    .with_system(handle_ui)
                    .with_system(connect_client)
                    .with_system(handle_connections)
                    .with_system(update_status)
                    .with_system(update_player_labels)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Lobby)
                    .with_system(clean_up)
            )
        ;
    }
}

fn setup_networking(
    mut commands: Commands,
    multiplayer_type: Res<MultiplayerType>,
    parts: Res<MsgTableParts>,
) {
    match *multiplayer_type {
        MultiplayerType::Server => {
            println!("server");
            commands.insert_resource(Server::new("127.0.0.1:5599".parse().unwrap(), parts.clone()).unwrap());
        }
        MultiplayerType::Host => {
            println!("host");
            commands.insert_resource(Server::new("127.0.0.1:5599".parse().unwrap(), parts.clone()).unwrap());
            commands.insert_resource(Client::new("127.0.0.1:5599".parse().unwrap(), parts.clone(), Connection::new("Host")).option());
        }
        MultiplayerType::Client => {
            println!("client");
            commands.insert_resource(Client::new("127.0.0.1:5599".parse().unwrap(), parts.clone(), Connection::new("Client")).option());
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
            println!("Client Connected!");
            if let Ok((client, resp)) = pending.take().unwrap().unwrap() {
                if let Response::Accepted(cid, optional_player) = resp {
                    if let Some(p) = optional_player {
                        players.add(cid, p);
                    }
                }
                commands.insert_resource(client);
            }
            commands.remove_resource::<OptionPendingClient>()
        }
    }
}

fn setup_lobby_ui(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    println!("Setting up lobby");
    let font = assets.load("FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font,
        color: Color::BLACK,
        font_size: 60.0,
    };
    let button_style = Style {
        size: Size::new(Val::Px(1000.0), Val::Px(100.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    // parent
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                padding: Rect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(LobbyItem)
        .with_children(|cb| {
            // Title
            cb.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect{ bottom: Val::Px(0.0), ..Rect::all(Val::Px(20.0))},
                    ..Default::default()
                },
                text: Text::with_section("Lobby", text_style.clone(), TextAlignment::default()),
                ..Default::default()
            });

            // Status
            cb.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(10.0)),
                    ..Default::default()
                },
                text: Text::with_section("Status: --", TextStyle { font_size: 40.0, ..text_style.clone() }, TextAlignment::default()),
                ..Default::default()
            }).insert(StatusLabel);

            // Players holder
            cb
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
                .with_children(|cb| {
                    cb.spawn_bundle(TextBundle {
                        text: Text::with_section("Player 1", text_style.clone(), TextAlignment::default()),
                        style: Style {
                            margin: Rect::all(Val::Px(40.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Player::One);

                    cb.spawn_bundle(TextBundle {
                        text: Text::with_section("Player 2", text_style.clone(), TextAlignment::default()),
                        style: Style {
                            margin: Rect::all(Val::Px(40.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(Player::Two);
                })
            ;
        })
    ;
}

fn update_status(
    mut q_status: Query<&mut Text, With<StatusLabel>>,
    multiplayer_type: Res<MultiplayerType>,
    client: Option<Res<Client>>,
    server: Option<Res<Server>>,
) {
    let status = format!("Status: {}",
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

fn update_player_labels(
    mut q_player_label: Query<(&mut Text, &Player)>,
    players: Res<Players>,
) {
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
    // q_interaction: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut game_state: ResMut<State<GameState>>,
) {
    // for (interaction, menu_button) in q_interaction.iter() {
    //     if *interaction == Interaction::Clicked {
    //         match menu_button {
    //             MenuButton::Server => game_state.set(GameState::Lobby(MultiplayerType::Server)).unwrap(),
    //             MenuButton::Host   => game_state.set(GameState::Lobby(MultiplayerType::Host)).unwrap(),
    //             MenuButton::Client => game_state.set(GameState::Lobby(MultiplayerType::Client)).unwrap(),
    //         }
    //     }
    // }
}

fn handle_connections(
    server: Option<ResMut<Server>>,
    client: Option<Res<Client>>,
    mut players: ResMut<Players>,
) {
    if let Some(mut server) = server {
        let mut broadcasts = vec![];
        server.handle_new_cons(&mut |cid, c| {
            let existing_player = players.p1.clone().map(|p| p.1);
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
        for msg in client.recv::<ConnectionBroadcast>().unwrap() {
            println!("Adding as client");
            players.add(msg.cid, msg.name.clone());
        }
    }
}

fn clean_up(
    mut commands: Commands,
    q_menu: Query<Entity, With<LobbyItem>>,
) {
    for e in q_menu.iter() {
        commands.entity(e).despawn_recursive();
    }
}
