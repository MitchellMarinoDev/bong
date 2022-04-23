use std::any::Any;
use bevy::prelude::*;
use carrier_pigeon::{MsgTable, Transport};
use serde::{Serialize, Deserialize};
use crate::plugin::net_comp::NetComp;

/// The synchronizing direction that data should be sent.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum NetDirection {
    /// Synchronize data from *down* from the peer, to this instance.
    Down,
    /// Synchronize data from this instance, *up* to the peer.
    Up,
}

/// A networked entity.
///
/// Any entity using [`NetComp`] needs to have one of these
#[derive(Component)]
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NetEntity {
    /// A unique identifier that needs to be the same on all connected instances of the game.
    /// A random `u64` provides a very low collision rate.
    id: u64,
}

/// The message type to be sent.
///
/// This wraps the component message type with the entity's `id`.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub(crate) struct NetCompMsg<M: Any + Send + Sync> {
    id: u64,
    msg: M,
}

/// An extension trait for easy registering [`NetComp`] types.
pub trait AppExt {
    /// Registers a Network Component type `T`.
    ///
    /// Adds the required system to the app, and registers `T` into the `table`.
    fn register_net_comp<T>(&mut self, table: &mut MsgTable) -> &mut Self
    where T: Any + Send + Send + Component;

    /// Registers a Network Component type `T` to be sent with type `M`.
    ///
    /// Adds the required system to the app, and registers `M` into the `table`.
    fn register_net_comp_custom<T, M>(&mut self, table: &mut MsgTable) -> &mut Self
    where
        T: Any + Send + Send + Into<M> + Component,
        M: Any + Send + Send,
    ;
}

impl AppExt for App {
    fn register_net_comp<T>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
    where T: Any + Send + Send + Component
    {
        table.register::<T>(transport);
        self.add_system(network_comp_sys::<T>)
    }

    fn register_net_comp_custom<T, M>(&mut self, table: &mut MsgTable) -> &mut Self
    where
        T: Any + Send + Send + Into<M> + Component,
        M: Any + Send + Send,
    {
        todo!()
    }
}

// TODO: needs access to the client or server.
fn network_comp_sys<T> (
    mut q: Query<(&NetEntity, &NetComp<T>, &mut T)>,
)
where T: Any + Send + Send + Component,
{
    for (net_e, net_c, comp) in q.iter_mut() {

    }
}

fn network_comp_sys_custom<T, M> (

)
where
    T: Any + Send + Send + Into<M> + Component,
    M: Any + Send + Send,
{

}
