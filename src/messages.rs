//! Contains the messages.
use bevy::math::Vec3Swizzles;
use bevy::prelude::Transform;
use carrier_pigeon::{CId, MsgTable, Transport};
use heron::Velocity;
use serde::{Deserialize, Serialize};

use crate::{default, Quat, Vec2};
use crate::game::Team;

/// The connection message.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Connection {
    pub name: String,
}

impl Connection {
    pub fn new(name: impl Into<String>) -> Self {
        Connection {
            name: name.into(),
        }
    }
}

/// The message that gets broadcast to other clients when one joins.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct ConnectionBroadcast {
    pub name: String,
    pub cid: CId,
}

impl ConnectionBroadcast {
    pub fn new(name: impl Into<String>, cid: CId) -> Self {
        ConnectionBroadcast {
            name: name.into(),
            cid
        }
    }
}

/// The message that gets broadcast to other clients when one disconnects.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DisconnectBroadcast {
    pub cid: CId,
}

/// The response message.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum Response {
    Accepted(CId, Option<(CId, String)>),
    Rejected(RejectReason),
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum RejectReason {
    MaxPlayersReached,
}

/// The disconnection message.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Disconnect {

}

pub fn get_table() -> MsgTable {
    let mut table = MsgTable::new();
    table.register::<ConnectionBroadcast>(Transport::TCP).unwrap();
    table.register::<DisconnectBroadcast>(Transport::TCP).unwrap();
    table.register::<StartGame>(Transport::TCP).unwrap();
    table.register::<BrickBreak>(Transport::TCP).unwrap();
    table.register::<GameWin>(Transport::TCP).unwrap();

    table
}

/// A message that indicates that the game has been started by the server.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct StartGame;

/// A message that indicates that the game has been started by the server.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct GameWin(pub Team);

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct BrickBreak(pub u32);

/// A reduced [`Transform`] component that can be networked.
///
/// Only holds fields relevant to this game.
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct MyTransform {
    pub translation: Vec2,
    pub rotation: Quat,
    // Rotation or scale are not used.
}

impl From<Transform> for MyTransform {
    fn from(o: Transform) -> Self {
        MyTransform {
            translation: o.translation.xy(),
            rotation: o.rotation,
        }
    }
}

impl From<MyTransform> for Transform {
    fn from(o: MyTransform) -> Self {
        Transform {
            translation: o.translation.extend(0.0),
            rotation: o.rotation,
            ..default()
        }
    }
}

/// A reduced [`Velocity`] component that can be networked.
///
/// Only holds fields relevant to this game.
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct MyVelocity {
    pub linear: Vec2,
    // angular velocity is not used.
}

impl From<Velocity> for MyVelocity {
    fn from(o: Velocity) -> Self {
        MyVelocity {
            linear: o.linear.xy(),
        }
    }
}

impl From<MyVelocity> for Velocity {
    fn from(o: MyVelocity) -> Self {
        Velocity {
            linear: o.linear.extend(0.0),
            ..default()
        }
    }
}