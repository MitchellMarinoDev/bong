use std::any::Any;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::Res;
use carrier_pigeon::{Client, Server};

pub fn run_if_client<C, R, D>(client: Option<Res<Client<C, R, D>>>) -> ShouldRun
where
    C: Any + Send + Sync,
    R: Any + Send + Sync,
    D: Any + Send + Sync,
{
    match client {
        Some(client) if client.open() => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn run_if_server<C, R, D>(server: Option<Res<Server<C, R, D>>>) -> ShouldRun
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    if server.is_some() { ShouldRun::Yes } else { ShouldRun::No }
}

pub fn run_if_host<C, R, D>(client: Option<Res<Client<C, R, D>>>, server: Option<Res<Server<C, R, D>>>) -> ShouldRun
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    if client.is_none() || server.is_none() { return ShouldRun::No }
    if client.unwrap().open() { ShouldRun::Yes } else { ShouldRun::No }
}