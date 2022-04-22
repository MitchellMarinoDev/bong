//! Contains the messages.
use serde::{Serialize, Deserialize};

/// The connection packet.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Connection {
    name: String,
    password: Option<String>,
}

impl Connection {
    pub fn new(name: impl Into<String>) -> Self {
        Connection {
            name: name.into(),
            password: None,
        }
    }

    pub fn new_pass(name: impl Into<String>, pass: Option<impl Into<String>>) -> Self {
        Connection {
            name: name.into(),
            password: pass.map(|i| i.into()),
        }
    }
}

/// The response packet.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Response {
    Accepted,
    Rejected(RejectReason),
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum RejectReason {
    MaxPlayersReached,
}

/// The disconnection packet.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Disconnect {

}
