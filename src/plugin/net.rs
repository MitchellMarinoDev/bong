use std::any::Any;

use bevy::prelude::*;
use carrier_pigeon::{CId, Client, MsgTable, Server, Transport};
use carrier_pigeon::net::CIdSpec;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::plugin::net_comp::NetComp;

/// The synchronizing direction for data.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum NetDirection {
    /// Synchronize data **to** the peer, from this instance.
    ///
    /// On a server, the [`CIdSpec`] is used to specify who to send the data to.
    To(CIdSpec),
    /// Synchronize data **from** the peer, to this instance.
    ///
    /// On a server, the [`CIdSpec`] is used to specify who to receive the data from.
    From(CIdSpec),
    /// Synchronize data to the peer, and form the peer. **This option is not valid on a client.**
    ///
    /// On a server, the [`CIdSpec`]s are used to specify who to send/receive the data to/from.
    ToFrom(CIdSpec, CIdSpec),
}

/// A networked entity.
///
/// Any entity using [`NetComp`] needs to have one of these
#[derive(Component)]
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NetEntity {
    /// A unique identifier that needs to be the same on all connected instances of the game.
    /// A random `u64` provides a very low collision rate.
    pub id: u64,
}

impl NetEntity {
    pub fn new(id: u64) -> Self {
        NetEntity {
            id
        }
    }
}

/// The message type to be sent.
///
/// This wraps the component message type with the entity's `id`.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub(crate) struct NetCompMsg<M: Any + Send + Sync> {
    // TODO: create type alias.
    id: u64,
    msg: M,
}

impl<M: Any + Send + Sync> NetCompMsg<M> {
    pub fn new(id: u64, msg: M) -> Self {
        NetCompMsg {
            id,
            msg,
        }
    }
}


/// An extension trait for easy registering [`NetComp`] types.
pub trait AppExt {
    fn register_net_comp<T, C, R, D>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
        where
            T: Any + Send + Sync + Serialize + DeserializeOwned + Component + Clone,
            C: Any + Send + Sync,
            R: Any + Send + Sync,
            D: Any + Send + Sync,
    ;

    fn register_net_comp_custom<T, M, C, R, D>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
    where
        T: Into<M> + Component + Clone,
        M: Any + Send + Sync + Into<T> + Serialize + DeserializeOwned + Clone,
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
    ;
}

impl AppExt for App {
    /// Registers the type `NetCompMsg<T>` into `table` and adds the
    /// system required to sync components of type `T`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<T>` is already registered.
    fn register_net_comp<T, C, R, D>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
    where
        T: Any + Send + Sync + Serialize + DeserializeOwned + Component + Clone,
        C: Any + Send + Sync,
        R: Any + Send + Sync,
        D: Any + Send + Sync,
    {
        // TODO: make a version that doesnt panic.
        table.register::<NetCompMsg<T>>(transport).unwrap();
        self.add_system(network_comp_sys::<T, C, R, D>)
    }

    /// Registers the type `NetCompMsg<M>` into `table` and adds the
    /// system required to sync components of type `T`,
    /// using type `M` to send.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered.
    fn register_net_comp_custom<T, M, C, R, D>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
        where
            T: Into<M> + Component + Clone,
            M: Any + Send + Sync + Into<T> + Serialize + DeserializeOwned + Clone,
            C: Any + Send + Sync,
            R: Any + Send + Sync,
            D: Any + Send + Sync,
    {
        // TODO: make a version that doesnt panic.
        table.register::<NetCompMsg<M>>(transport).unwrap();
        self.add_system(network_comp_sys_custom::<T, M, C, R, D>)
    }
}

// Syncing Systems
fn network_comp_sys<T, C, R, D> (
    server: Option<ResMut<Server<C, R, D>>>,
    client: Option<ResMut<Client<C, R, D>>>,
    mut q: Query<(&NetEntity, &NetComp<T>, &mut T)>,
    // Add option for sending changed only.
)
where
    T: Any + Send + Sync + Serialize + DeserializeOwned + Component + Clone,
    C: Any + Send + Sync,
    R: Any + Send + Sync,
    D: Any + Send + Sync,
{
    if let Some(mut server) = server {
        let msgs: Vec<(CId, &NetCompMsg<T>)> = server.recv::<NetCompMsg<T>>().unwrap().collect();
        for (net_e, net_c, mut comp) in q.iter_mut() {
            match net_c.dir {
                NetDirection::From(spec) => {
                    // Get the last message that matches with the entity and CIdSpec
                    if let Some(&(_cid, valid_msg)) = msgs.iter().filter(|(cid, msg)| spec.matches(*cid) && msg.id == net_e.id).last() {
                        *comp = valid_msg.msg.clone();
                    }
                }
                NetDirection::To(spec) => {
                    // TODO: handle potential error.
                    server.send_spec(&NetCompMsg::<T>::new(net_e.id, comp.clone()), spec).unwrap();
                }
                NetDirection::ToFrom(to_spec, from_spec) => {
                    todo!()
                }
            }
        }
    } else if let Some(mut client) = client {
        let msgs: Vec<&NetCompMsg<T>> = client.recv::<NetCompMsg<T>>().unwrap().collect();
        for (net_e, net_c, mut comp) in q.iter_mut() {
            match net_c.dir {
                NetDirection::From(_) => {
                    // Get the last message that matches with the entity and CIdSpec
                    if let Some(&valid_msg) = msgs.iter().filter(|msg| msg.id == net_e.id).last() {
                        *comp = valid_msg.msg.clone();
                    }
                }
                NetDirection::To(_) => {
                    // TODO: handle potential error.
                    client.send(&NetCompMsg::<T>::new(net_e.id, comp.clone())).unwrap();
                }
                NetDirection::ToFrom(_, _) => {
                    todo!()
                }
            }
        }
    }
}

fn network_comp_sys_custom<T, M, C, R, D> (
    server: Option<ResMut<Server<C, R, D>>>,
    client: Option<ResMut<Client<C, R, D>>>,
    mut q: Query<(&NetEntity, &NetComp<T, M>, &mut T)>,
    // Add option for sending changed only.
)
where
    T: Into<M> + Component + Clone,
    M: Any + Send + Sync + Into<T> + Serialize + DeserializeOwned + Clone,
    C: Any + Send + Sync,
    R: Any + Send + Sync,
    D: Any + Send + Sync,
{
    if let Some(mut server) = server {
        let msgs: Vec<(CId, &NetCompMsg<M>)> = server.recv::<NetCompMsg<M>>().unwrap().collect();
        for (net_e, net_c, mut comp) in q.iter_mut() {
            match net_c.dir {
                NetDirection::From(spec) => {
                    // Get the last message that matches with the entity and CIdSpec
                    if let Some(&(_cid, valid_msg)) = msgs.iter().filter(|(cid, msg)| spec.matches(*cid) && msg.id == net_e.id).last() {
                        *comp = valid_msg.msg.clone().into();
                    }
                }
                NetDirection::To(spec) => {
                    // TODO: handle potential error.
                    server.send_spec(&NetCompMsg::<M>::new(net_e.id, comp.clone().into()), spec).unwrap();
                }
                NetDirection::ToFrom(to_spec, from_spec) => {
                    todo!()
                }
            }
        }
    } else if let Some(mut client) = client {
        let msgs: Vec<&NetCompMsg<M>> = client.recv::<NetCompMsg<M>>().unwrap().collect();
        for (net_e, net_c, mut comp) in q.iter_mut() {
            match net_c.dir {
                NetDirection::From(_) => {
                    // Get the last message that matches with the entity and CIdSpec
                    if let Some(&valid_msg) = msgs.iter().filter(|msg| msg.id == net_e.id).last() {
                        *comp = valid_msg.msg.clone().into();
                    }
                }
                NetDirection::To(_) => {
                    // TODO: handle potential error.
                    client.send(&NetCompMsg::<M>::new(net_e.id, comp.clone().into())).unwrap();
                }
                NetDirection::ToFrom(_, _) => {
                    todo!()
                }
            }
        }
    }
}
