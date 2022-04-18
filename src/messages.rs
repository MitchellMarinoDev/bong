//! Contains the messages.
use serde::{Serialize, Deserialize};

/// The connection packet.
#[derive(Serialize, Deserialize)]
pub struct Connection {
    password: Option<String>,
}

/// The response packet.
#[derive(Serialize, Deserialize)]
pub enum Response {
    Accepted,
    Rejected(RejectReason),
}

#[derive(Serialize, Deserialize)]
pub enum RejectReason {
    MaxPlayersReached,
}

/// The disconnection packet.
#[derive(Serialize, Deserialize)]
pub struct Disconnect {

}
