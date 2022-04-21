use std::any::Any;
use std::marker::PhantomData;
use bevy::prelude::*;
use crate::plugin::recv::{client_recv, server_recv};

mod recv;
mod run_criteria;

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
            .add_system_to_stage(CoreStage::First, client_recv::<C, R, D>)
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
            .add_system_to_stage(CoreStage::First, server_recv::<C, R, D>)
        ;
    }
}

impl<C, R, D> ServerPlugin<C, R, D>
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C, R, D> ClientPlugin<C, R, D>
    where
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
{
    pub fn new() -> Self {
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
