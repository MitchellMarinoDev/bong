use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use carrier_pigeon::net::CIdSpec;
use CIdSpec::All;
use heron::*;
use NetDirection::*;
use crate::{Client, GameState, MultiplayerType, MyTransform, MyVelocity, Server};
use crate::messages::BrickBreak;
use crate::plugin::{NetComp, NetDirection, NetEntity};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup_game)
                    .with_system(setup_bricks)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(break_bricks)
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
pub enum Team {
    Left,
    Right,
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
    println!("Dir: {:?}", dir);

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
        .insert(Velocity::from_linear(Vec3::new(750.0, -500.0, 0.0)))
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
        let ball = q_ball.single();
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
    } else if let Some(client) = client {
        let ids: Vec<_> = client.recv::<BrickBreak>().unwrap().map(|m| m.0).collect();
        for (e, brick) in q_brick.iter() {
            if ids.contains(&brick.0) {
                commands.entity(e).despawn();
            }
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
}