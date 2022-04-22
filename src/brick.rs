use bevy::prelude::*;
use heron::*;
use crate::{Ball, Brick};

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_bricks)
            .add_system(break_bricks)
        ;
    }
}

fn setup_bricks(
    mut commands: Commands,
) {
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
            spawn_brick(&mut commands, color, [x,  (h * height)].into(), width, height);
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
            spawn_brick(&mut commands, color, [x,  (h * height)].into(), width, height);
        }
    }
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