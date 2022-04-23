use std::any::Any;
use std::marker::PhantomData;
use bevy::prelude::*;
use crate::plugin::tick::{client_tick, server_tick};

mod tick;
mod net_comp;
mod net;

pub use net::AppExt;

pub struct ClientPlugin <
    C: Any + Send + Sync,
    R: Any + Send + Sync,
    D: Any + Send + Sync,
> (PhantomData<(C, R, D)>);

pub struct ServerPlugin <
    C: Any + Send + Sync,
    R: Any + Send + Sync,
    D: Any + Send + Sync,
> (PhantomData<(C, R, D)>);

impl<C, R, D> Plugin for ClientPlugin<C, R, D>
where
    C: Any + Send + Sync,
    R: Any + Send + Sync,
    D: Any + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::First, client_tick::<C, R, D>)
        ;
    }
}

impl<C, R, D> Plugin for ServerPlugin<C, R, D>
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::First, server_tick::<C, R, D>)
        ;
    }
}

impl<C, R, D> Default for ServerPlugin<C, R, D>
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C, R, D> Default for ClientPlugin<C, R, D>
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}
