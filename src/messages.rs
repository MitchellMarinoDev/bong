//! Contains the messages.
use serde::{Serialize, Deserialize};

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
}

impl ConnectionBroadcast {
    pub fn new(name: impl Into<String>) -> Self {
        ConnectionBroadcast {
            name: name.into(),
        }
    }
}

/// The response message.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Response {
    Accepted,
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
