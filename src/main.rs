mod plugin;
mod messages;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::messages::{Connection, Disconnect, Response};
use crate::plugin::ClientPlugin;
use bevy::render::camera::ScalingMode;
use bevy_rapier2d::na::Vector2;

type Client = carrier_pigeon::Client<Connection, Response, Disconnect>;
type Server = carrier_pigeon::Server<Connection, Response, Disconnect>;

fn main() {
    App::new()
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_plugin(ClientPlugin::<Connection, Response, Disconnect>::new())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    camera.orthographic_projection.left = -1920.0/2.0;
    camera.orthographic_projection.right = 1920.0/2.0;
    camera.orthographic_projection.bottom = -1080.0/2.0;
    camera.orthographic_projection.top = 1080.0/2.0;

    commands.spawn()
        .insert_bundle(camera);

    commands.spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(100.0, 200.0)),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(255, 50, 50),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..Default::default()
        })
        .insert(Brick);

    // ball
    commands.spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::ball(10.0)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
            velocity: RigidBodyVelocityComponent(
                RigidBodyVelocity::new(Vector::new(1.0, 1.0), Default::default())
            ),
            mass_properties: Default::default(),
            forces: Default::default(),
            activation: Default::default(),
            damping: Default::default(),
            dominance: Default::default(),
            ccd: Default::default(),
            changes: Default::default(),
            ids: Default::default(),
            colliders: Default::default(),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(255, 50, 50),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-495.0, 500.0, 0.0),
            ..Default::default()
        });
}

#[derive(Component)]
struct Brick;

