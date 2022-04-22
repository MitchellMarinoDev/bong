use bevy::prelude::*;
use crate::{Client, Connection, GameState, MsgTableParts, MultiplayerType, Server};

pub struct LobbyPlugin;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// All lobby items have this so that they can be cleaned up easily.
struct LobbyItem;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// The player marker
enum Player {
    One,
    Two,
}

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Lobby)
                    .with_system(setup_networking)
                    .with_system(setup_lobby_ui)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Lobby)
                    .with_system(handle_ui)
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
            commands.insert_resource(Client::new("127.0.0.1:5599".parse().unwrap(), parts.clone(), Connection::new("name")));
        }
        MultiplayerType::Client => {
            println!("client");
            commands.insert_resource(Client::new("127.0.0.1:5599".parse().unwrap(), parts.clone(), Connection::new("name")));
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
                    margin: Rect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                text: Text::with_section("Lobby", text_style.clone(), TextAlignment::default()),
                ..Default::default()
            });

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

fn clean_up(
    mut commands: Commands,
    q_menu: Query<Entity, With<LobbyItem>>,
) {
    for e in q_menu.iter() {
        commands.entity(e).despawn_recursive();
    }
}
