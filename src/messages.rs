//! Contains the messages.
use carrier_pigeon::{CId, MsgTable, Transport};
use serde::{Serialize, Deserialize};
use crate::MsgTableParts;

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
    Accepted(CId, Option<String>),
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

pub fn get_parts() -> MsgTableParts {
    let mut table = MsgTable::new();
    table.register::<ConnectionBroadcast>(Transport::TCP).unwrap();
    table.register::<DisconnectBroadcast>(Transport::TCP).unwrap();

    table.build().unwrap()
}