mod plugin;
mod messages;
mod brick;

use bevy::prelude::*;
use crate::messages::{Connection, Disconnect, Response};
use crate::plugin::ClientPlugin;
use bevy::render::camera::ScalingMode;
use heron::prelude::*;
use crate::brick::BrickPlugin;

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
        .add_plugin(BrickPlugin)

        .add_plugin(ClientPlugin::<Connection, Response, Disconnect>::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
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

    let ball_ico = assets.load("ball.png");
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


#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Target(Team);
