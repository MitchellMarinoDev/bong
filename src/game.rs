use std::f32::consts::PI;
use std::time::{Duration, Instant};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use carrier_pigeon::net::CIdSpec;
use carrier_pigeon::{Client, Server};
use carrier_pigeon::net::CIdSpec::{Except, Only};
use CIdSpec::All;
use heron::*;
use NetDirection::*;
use crate::{GameState, MyTransform, MyVelocity};
use crate::lobby::Players;
use crate::messages::{BrickBreak, GameWin};
use crate::plugin::{NetComp, NetDirection, NetEntity};
use serde::{Serialize, Deserialize};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup_game)
                    .with_system(setup_bricks)
                    .with_system(setup_paddles)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(break_bricks)
                    .with_system(move_paddle)
                    .with_system(clamp_ball_speed)
                    .with_system(game_win)
                    .with_system(leave_game_after_win)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(clean_up)
            )
        ;
    }
}

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// A brick that is destroyed when the ball hits it.
pub struct Brick(pub u32);

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// A brick that is destroyed when the ball hits it.
pub struct Paddle(Team);

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// The ball.
pub struct Ball;

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// The target that the opposing team is trying to hit.
pub struct Target(Team);

#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// All game items have this so that they can be cleaned up easily.
pub struct GameItem;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// All game items have this so that they can be cleaned up easily.
pub struct GameWinR(pub Instant);


#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Team {
    Left,
    Right,
}

impl Team {
    fn other(&self) -> Self {
        match self {
            Team::Left => Team::Right,
            Team::Right => Team::Left,
        }
    }
}

fn setup_game(
    mut commands: Commands,
    server: Option<Res<Server>>,
    assets: Res<AssetServer>,
) {
    // Walls
    commands.spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(RigidBody::Static)
        .insert(GameItem)
        .insert(Name::new("Walls"))
        .with_children(|parent| {
            // Bottom
            parent.spawn()
                .insert(Name::new("Wall B"))
                .insert(Transform::from_xyz(0.0, (-1080.0/2.0)-40.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(2000.0/2.0, 40.0, 0.0), border_radius: None });

            // Top
            parent.spawn()
                .insert(Name::new("Wall T"))
                .insert(Transform::from_xyz(0.0, (1080.0/2.0)+40.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(2000.0/2.0, 40.0, 0.0), border_radius: None });

            // Left
            parent.spawn()
                .insert(Name::new("Wall L"))
                .insert(Transform::from_xyz((-1920.0/2.0)-40.0, 0.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(40.0, 1160.0/2.0, 0.0), border_radius: None });

            // Right
            parent.spawn()
                .insert(Name::new("Wall R"))
                .insert(Transform::from_xyz((1920.0/2.0)+40.0, 0.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(40.0, 1160.0/2.0, 0.0), border_radius: None });

        })
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
    ;

    let ball_ico = assets.load("ball.png");

    // ball
    let dir = if server.is_some() { To(All) } else { From(All) };
    let velocity_comp = NetComp::<Velocity, MyVelocity>::new(dir);
    let transform_comp = NetComp::<Transform, MyTransform>::new(dir);

    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(255, 50, 50),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            texture: ball_ico,
            // transform: Transform::from_xyz(-500.0, 500.0, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(RigidBody::Dynamic)
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
        .insert(RotationConstraints::lock())
        .insert(Velocity::from_linear(Vec3::new(750.0, 0.0, 0.0)))
        .insert(GameItem)
        .insert(Ball)
        .insert(Name::new("Ball"))

        .insert(velocity_comp)
        .insert(transform_comp)
        .insert(NetEntity::new(5768696975200910899))
    ;

    // Targets
    let crown_ico = assets.load("crown.png");
    let target_size = 125.0;
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(255, 25, 25),
                custom_size: Some(Vec2::new(target_size, target_size)),
                ..Default::default()
            },
            texture: crown_ico.clone(),
            transform: Transform::from_xyz(-897.5, 0.0, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::Sphere { radius: target_size/2.0 })
        .insert(RigidBody::Sensor)
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
        .insert(RotationConstraints::lock())
        .insert(Collisions::default())
        .insert(GameItem)
        .insert(Target(Team::Left))
        .insert(Name::new("Left Target"))
    ;

    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(25, 25, 255),
                custom_size: Some(Vec2::new(target_size, target_size)),
                ..Default::default()
            },
            texture: crown_ico,
            transform: Transform::from_xyz(897.5, 0.0, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::Sphere { radius: target_size/2.0 })
        .insert(RigidBody::Sensor)
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
        .insert(RotationConstraints::lock())
        .insert(Collisions::default())
        .insert(GameItem)
        .insert(Target(Team::Right))
        .insert(Name::new("Left Right"))
    ;
}

fn setup_bricks(
    mut commands: Commands,
) {
    let mut id = 0;
    let mut bricks = vec![];
    // Left
    let height = 108.0;
    let width = 60.0;
    let count = 10;
    let colors = [Color::RED, Color::ORANGE_RED, Color::ORANGE, Color::YELLOW, Color::YELLOW_GREEN, Color::GREEN];

    for r in 0..colors.len() {
        let color = colors[r];
        let x = -500.0 - width * r as f32;
        for i in 1..=count {
            let h = i as f32 - (count+1) as f32 / 2.0;
            bricks.push(spawn_brick(&mut commands, color, [x,  (h * height)].into(), width, height, id));
            id += 1;
        }
    }

    // Right
    let height = 108.0;
    let width = 60.0;
    let count = 10;
    let colors = [Color::SEA_GREEN, Color::BLUE, Color::MIDNIGHT_BLUE, Color::INDIGO, Color::PURPLE, Color::VIOLET];

    for r in 0..colors.len() {
        let color = colors[r];
        let x = 500.0 + width * r as f32;
        for i in 1..=count {
            let h = i as f32 - (count+1) as f32 / 2.0;
            bricks.push(spawn_brick(&mut commands, color, [x,  (h * height)].into(), width, height, id));
            id += 1;
        }
    }

    commands.spawn()
        .insert(Name::new("Bricks"))
        .insert(GlobalTransform::identity())
        .insert(Transform::identity())
        .insert(GameItem)
        .push_children(&bricks[..]);
}

fn setup_paddles(
    server: Option<Res<Server>>,
    players: Res<Players>,
    mut commands: Commands,
) {
    let width = 30.0;
    let height = 200.0;

    let p1 = players.p1.as_ref().unwrap().0;
    let p2 = players.p2.as_ref().unwrap().0;

    let left_dir;
    let right_dir;

    if server.is_some() {
        // Server
        left_dir = NetDirection::ToFrom(Except(p1), Only(p1));
        right_dir = NetDirection::ToFrom(Except(p2), Only(p2));
    } else {
        match players.me.unwrap() {
            Team::Left => {
                left_dir = NetDirection::to();
                right_dir = NetDirection::from();
            },
            Team::Right => {
                left_dir = NetDirection::from();
                right_dir = NetDirection::to();
            },
        }
    }

    // Left
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                color: Color::RED,
                ..Default::default()
            },
            transform: Transform::from_xyz(-350.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::Cuboid { half_extends: Vec3::new(width/2.0, height/2.0, 0.0), border_radius: None })
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
        .insert(RigidBody::KinematicPositionBased)
        .insert(RotationConstraints::restrict_to_z_only())
        .insert(GameItem)
        .insert(Paddle(Team::Left))
        .insert(NetEntity::new(6413180502345645314))
        .insert(NetComp::<Transform, MyTransform>::new(left_dir))
        .insert(Name::new("Paddle L"));

    // Right
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                color: Color::BLUE,
                ..Default::default()
            },
            transform: Transform::from_xyz(350.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::Cuboid { half_extends: Vec3::new(width/2.0, height/2.0, 0.0), border_radius: None })
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
        .insert(RigidBody::KinematicPositionBased)
        .insert(RotationConstraints::restrict_to_z_only())
        .insert(GameItem)
        .insert(Paddle(Team::Right))
        .insert(NetEntity::new(6413180502345645315))
        .insert(NetComp::<Transform, MyTransform>::new(right_dir))
        .insert(Name::new("Paddle R"));
}

fn move_paddle(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    players: Res<Players>,
    mut q_paddle: Query<(&mut Transform, &Paddle)>,
) {
    // Only run if we are a player.
    if players.me.is_none() { return; }
    let me = players.me.unwrap();

    let mut paddle: (Mut<Transform>, &Paddle) = q_paddle.iter_mut().filter(|(_t, p)| p.0 == me).next().unwrap();
    let mut translation = paddle.0.translation;
    let mut rotation = paddle.0.rotation;

    if input.pressed(KeyCode::W) {
        translation += Vec3::new(0.0, 14.0, 0.0) * time.delta_seconds() * 60.0;
    }

    if input.pressed(KeyCode::S) {
        translation -= Vec3::new(0.0, 14.0, 0.0) * time.delta_seconds() * 60.0;
    }

    let (x, y, mut z) = rotation.to_euler(EulerRot::XYZ);

    if input.pressed(KeyCode::Q) {
        z += PI/72.0 * time.delta_seconds() * 60.0;
    }
    if input.pressed(KeyCode::E) {
        z -= PI/72.0 * time.delta_seconds() * 60.0;
    }

    // Clamp
    z = z.clamp(-PI/8.0, PI/8.0);
    rotation = Quat::from_euler(EulerRot::XYZ, x, y ,z);

    translation.y = translation.y.clamp(-500.0, 500.0);

    // Apply
    paddle.0.rotation = rotation;
    paddle.0.translation = translation;
}

fn break_bricks(
    server: Option<Res<Server>>,
    client: Option<Res<Client>>,
    q_ball: Query<Entity, With<Ball>>,
    q_brick: Query<(Entity, &Brick)>,
    mut collisions: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    if let Some(server) = server {
        // Break balls based on collision
        if let Some(ball) = q_ball.iter().next() {
            for event in collisions.iter() {
                if let CollisionEvent::Stopped(d1, d2) = event {
                    let e1 = d1.rigid_body_entity();
                    let e2 = d2.rigid_body_entity();

                    let brick_e2: Result<(Entity, &Brick), QueryEntityError> = q_brick.get(e2);
                    let brick_e1: Result<(Entity, &Brick), QueryEntityError> = q_brick.get(e1);

                    // e2 is a brick colliding with a ball
                    if e1 == ball && brick_e2.is_ok() {
                        server.broadcast(&BrickBreak(brick_e2.unwrap().1.0)).unwrap();
                        commands.entity(e2).despawn();
                    }
                    // e1 is a brick colliding with a ball
                    if e2 == ball && brick_e1.is_ok() {
                        server.broadcast(&BrickBreak(brick_e1.unwrap().1.0)).unwrap();
                        commands.entity(e1).despawn();
                    }
                }
            }
        }
    } else if let Some(client) = client {
        let ids: Vec<_> = client.recv::<BrickBreak>().unwrap().map(|m| m.0).collect();
        for (e, brick) in q_brick.iter() {
            if ids.contains(&brick.0) {
                commands.entity(e).despawn();
            }
        }
    }
}

fn clamp_ball_speed(
    mut q_ball: Query<&mut Velocity, With<Ball>>,
) {
    if let Some(mut ball) = q_ball.iter_mut().next() {
        ball.linear.x = ball.linear.x.clamp(200.0, 100000.0);
        if ball.linear.length() < 400.0 {
            ball.linear = ball.linear.normalize() * 400.0;
        }
        if ball.linear.length() > 1200.0 {
            ball.linear = ball.linear.normalize() * 1200.0;
        }
    }
}

fn game_win(
    players: Res<Players>,
    server: Option<Res<Server>>,
    client: Option<Res<Client>>,
    q_targets: Query<(&Target, &Collisions)>,
    q_ball: Query<Entity, With<Ball>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    if let Some(server) = server {
        for (target, collisions) in q_targets.iter() {
            let collisions: &Collisions = collisions;
            for e in collisions.entities() {
                if q_ball.get(e).is_ok() {
                    commands.insert_resource(GameWinR(Instant::now()));

                    let font = assets.load("FiraMono-Medium.ttf");

                    let win_side = target.0.other();
                    let winner = match win_side {
                        Team::Left => players.p1.as_ref().unwrap().1.clone(),
                        Team::Right => players.p2.as_ref().unwrap().1.clone(),
                    };
                    server.broadcast(&GameWin(win_side)).unwrap();
                    commands.entity(e).despawn();
                    commands.spawn_bundle(TextBundle {
                        node: Default::default(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            margin: Rect::all(Val::Auto),
                            padding: Rect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            align_items: AlignItems::Center,
                            align_self: AlignSelf::Center,
                            size: Size { width: Val::Percent(100.0), height: Val::Auto },
                            ..default()
                        },
                        text: Text::with_section(
                            format!("{winner} wins!"),
                            TextStyle {
                                font,
                                font_size: 60.0,
                                color: Color::BLACK,
                            },
                            TextAlignment::default(),
                        ),
                        ..default()
                    }).insert(GameItem);
                }
            }
        }
    } else if let Some(client) = client {
        for gw in client.recv::<GameWin>().unwrap() {
            commands.insert_resource(GameWinR(Instant::now()));

            let font = assets.load("FiraMono-Medium.ttf");

            let win_side = gw.0;
            let winner = match win_side {
                Team::Left => players.p1.as_ref().unwrap().1.clone(),
                Team::Right => players.p2.as_ref().unwrap().1.clone(),
            };

            let ball = q_ball.iter().next().unwrap();
            commands.entity(ball).despawn();

            commands.spawn_bundle(TextBundle {
                node: Default::default(),
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: Rect::all(Val::Auto),
                    padding: Rect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    size: Size { width: Val::Percent(100.0), height: Val::Auto },
                    ..default()
                },
                text: Text::with_section(
                    format!("{winner} wins!"),
                    TextStyle {
                        font,
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                    TextAlignment::default(),
                ),
                ..default()
            }).insert(GameItem);
        }
    }
}

fn leave_game_after_win(
    game_win: Option<Res<GameWinR>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if let Some(gw) = game_win {
        if gw.0.elapsed() > Duration::from_millis(3000) {
            let _ = game_state.set(GameState::Menu);
        }
    }
}

fn spawn_brick(commands: &mut Commands, color: Color, center: Vec2, width: f32, height: f32, id: u32) -> Entity {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                color,
                ..Default::default()
            },
            transform: Transform::from_xyz(center.x, center.y, 0.0),
            ..Default::default()
        })
        .insert(CollisionShape::Cuboid { half_extends: Vec3::new(width/2.0, height/2.0, 0.0), border_radius: None })
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
        .insert(RigidBody::Static)
        .insert(GameItem)
        .insert(Brick(id))
        .insert(Name::new("Brick"))
        .id()
}

fn clean_up(
    mut commands: Commands,
    q_game_items: Query<Entity, With<GameItem>>,
) {
    for e in q_game_items.iter() {
        commands.entity(e).despawn_recursive();
    }
    commands.remove_resource::<Client>();
    commands.remove_resource::<Server>();
    commands.remove_resource::<GameWinR>();
}