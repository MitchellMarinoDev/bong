use bevy::prelude::*;
use std::any::Any;
use carrier_pigeon::{Client, Server};

pub fn client_tick<C, R, D> (
    client: Option<ResMut<Client<C, R, D>>>,
)
where
    C: Any + Send + Sync,
    R: Any + Send + Sync,
    D: Any + Send + Sync,
{
    if let Some(mut client) = client {
        client.clear_msgs();
        client.recv_msgs();
    }
}

pub fn server_tick<C, R, D> (
    server: Option<ResMut<Server<C, R, D>>>,
)
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    if let Some(mut server) = server {
        server.clear_msgs();
        server.recv_msgs();
    }
}