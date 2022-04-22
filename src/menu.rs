use bevy::prelude::*;
use crate::{GameState, MultiplayerType};

pub struct MenuPlugin;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// All menu items have this so that they can be cleaned up easily.
struct MenuItem;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum MenuButton {
    Server,
    Host,
    Client,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(handle_ui)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(clean_up)
            )
        ;
    }
}

fn setup_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
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

    println!("setting up menu");
    // parent
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(MenuItem)
        .with_children(|cb| {
            cb
                .spawn_bundle(ButtonBundle {
                    color: UiColor(Color::rgb_u8(255, 255, 255)),
                    style: button_style.clone(),
                    // transform: Transform::from_xyz(100.0, 0.0, 0.0),
                    ..Default::default()
                })
                .insert(MenuButton::Server)
                .with_children(|cb| {
                    cb.spawn_bundle(TextBundle {
                        text: Text::with_section("Start Server", text_style.clone(), TextAlignment::default()),
                        ..Default::default()
                    });
                });

            cb
                .spawn_bundle(ButtonBundle {
                    color: UiColor(Color::rgb_u8(255, 255, 255)),
                    style: button_style.clone(),
                    // transform: Transform::from_xyz(100.0, 0.0, 0.0),
                    ..Default::default()
                })
                .insert(MenuButton::Host)
                .with_children(|cb| {
                    cb.spawn_bundle(TextBundle {
                        text: Text::with_section("Start Host", text_style.clone(), TextAlignment::default()),
                        ..Default::default()
                    });
                });

            cb
                .spawn_bundle(ButtonBundle {
                    color: UiColor(Color::rgb_u8(255, 255, 255)),
                    style: button_style,
                    // transform: Transform::from_xyz(100.0, 0.0, 0.0),
                    ..Default::default()
                })
                .insert(MenuButton::Client)
                .with_children(|cb| {
                    cb.spawn_bundle(TextBundle {
                        text: Text::with_section("Start Client", text_style, TextAlignment::default()),
                        ..Default::default()
                    });
                });
        })
    ;
}

fn handle_ui(
    q_interaction: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button) in q_interaction.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button {
                MenuButton::Server => game_state.set(GameState::Lobby(MultiplayerType::Server)).unwrap(),
                MenuButton::Host   => game_state.set(GameState::Lobby(MultiplayerType::Host)).unwrap(),
                MenuButton::Client => game_state.set(GameState::Lobby(MultiplayerType::Client)).unwrap(),
            }
        }
    }
}

fn clean_up(
    mut commands: Commands,
    q_menu: Query<Entity, With<MenuItem>>,
) {
    for e in q_menu.iter() {
        commands.entity(e).despawn_recursive();
    }
}