mod plugin;
mod messages;

use bevy::prelude::*;
use crate::messages::{Connection, Disconnect, Response};
use crate::plugin::ClientPlugin;
use bevy::render::camera::ScalingMode;
use bevy::render::render_resource::Texture;
use carrier_pigeon::Header;
use heron::prelude::*;

type Client = carrier_pigeon::Client<Connection, Response, Disconnect>;
type Server = carrier_pigeon::Server<Connection, Response, Disconnect>;

enum Team {
    Left,
    Right,
}

fn main() {
    App::new()
        // Plugins
        .insert_resource(WindowDescriptor {
            title: "Bong".into(),
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())

        .add_plugin(ClientPlugin::<Connection, Response, Disconnect>::default())
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_system(break_bricks)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let ball_ico = assets.load("ball.png");
    let crown_ico = assets.load("crown.png");

    // Camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.left = -1920.0/2.0;
    camera.orthographic_projection.right = 1920.0/2.0;
    camera.orthographic_projection.bottom = -1080.0/2.0;
    camera.orthographic_projection.top = 1080.0/2.0;
    commands.spawn()
        .insert_bundle(camera);

    // Walls
    commands.spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(RigidBody::Static)
        .with_children(|cb| {
            // Bottom
            cb.spawn()
                .insert(Transform::from_xyz(0.0, (-1080.0/2.0)-40.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(2000.0/2.0, 40.0, 0.0), border_radius: None });

            // Top
            cb.spawn()
                .insert(Transform::from_xyz(0.0, (1080.0/2.0)+40.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(2000.0/2.0, 40.0, 0.0), border_radius: None });

            // Left
            cb.spawn()
                .insert(Transform::from_xyz((-1920.0/2.0)-40.0, 0.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(40.0, 1160.0/2.0, 0.0), border_radius: None });

            // Right
            cb.spawn()
                .insert(Transform::from_xyz((1920.0/2.0)+40.0, 0.0, 0.0))
                .insert(GlobalTransform::default())
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(40.0, 1160.0/2.0, 0.0), border_radius: None });

        })
        .insert(PhysicMaterial { restitution: 1.0, ..Default::default() })
    ;

    // Bricks Left
    let height = 108.0;
    let width = 60.0;
    let count = 10;
    let colors = [Color::RED, Color::ORANGE_RED, Color::ORANGE, Color::YELLOW, Color::YELLOW_GREEN, Color::GREEN];

    for r in 0..colors.len() {
        let color = colors[r];
        let x = -500.0 - width * r as f32;
        for i in 1..=count {
            let h = i as f32 - (count+1) as f32 / 2.0;
            spawn_brick(&mut commands, color, [x,  (h * height)].into(), width, height);
        }
    }

    // Bricks Right
    let height = 108.0;
    let width = 60.0;
    let count = 10;
    let colors = [Color::SEA_GREEN, Color::BLUE, Color::MIDNIGHT_BLUE, Color::INDIGO, Color::PURPLE, Color::VIOLET];

    for r in 0..colors.len() {
        let color = colors[r];
        let x = 500.0 + width * r as f32;
        for i in 1..=count {
            let h = i as f32 - (count+1) as f32 / 2.0;
            spawn_brick(&mut commands, color, [x,  (h * height)].into(), width, height);
        }
    }

    // ball
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
        .insert(Ball)
    ;

    // Targets
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
        .insert(Target(Team::Left))
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
        .insert(Target(Team::Right))
    ;
}

fn break_bricks(
    q_ball: Query<Entity, With<Ball>>,
    q_brick: Query<(), With<Brick>>,
    mut collisions: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    let ball = q_ball.single();
    for event in collisions.iter() {
        match event {
            CollisionEvent::Stopped(d1, d2) => {
                let e1 = d1.rigid_body_entity();
                let e2 = d2.rigid_body_entity();

                // e2 is a brick colliding with a ball
                if e1 == ball && q_brick.get(e2).is_ok() {
                    commands.entity(e2).despawn();
                }
                // e1 is a brick colliding with a ball
                if e2 == ball && q_brick.get(e1).is_ok() {
                    commands.entity(e1).despawn();
                }
            },
            _ => {},
        }
    }
}

fn spawn_brick(mut commands: &mut Commands, color: Color, center: Vec2, width: f32, height: f32) {
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
        .insert(Brick)
    ;
}

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Target(Team);
