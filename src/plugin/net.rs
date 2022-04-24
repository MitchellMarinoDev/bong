use std::any::Any;

use bevy::prelude::*;
use carrier_pigeon::{CId, Client, MsgRegError, MsgTable, Server, Transport};
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
    fn sync_comp<T, M>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
        where
            T: Clone + Into<M> + Component,
            M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    ;

    fn try_sync_comp<T, M>(&mut self, table: &mut MsgTable, transport: Transport) -> Result<&mut Self, MsgRegError>
        where
            T: Clone + Into<M> + Component,
            M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    ;
}

impl AppExt for App {
    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Registers the type `NetCompMsg<M>` into `table` and adds the
    /// system required to sync components of type `T`, using type `M`
    /// to send.
    ///
    /// Types `T` and `M` ***can*** be the same type; if the component `T`
    /// implements all the required traits, you may use it as `M`.
    ///
    /// ### Panics
    /// panics if `NetCompMsg<M>` is already registered in the table (If you
    /// call this method twice with the same `M`).
    fn sync_comp<T, M>(&mut self, table: &mut MsgTable, transport: Transport) -> &mut Self
    where
        T: Clone + Into<M> + Component,
        M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        table.register::<NetCompMsg<M>>(transport).unwrap();
        self.add_system(network_comp_sys::<T, M>)
    }

    /// Adds everything needed to sync component `T` using message type `M`.
    ///
    /// Same as [`sync_comp()`](App::sync_comp), but doesnt panic in the event of a [`MsgRegError`].
    fn try_sync_comp<T, M>(&mut self, table: &mut MsgTable, transport: Transport) -> Result<&mut Self, MsgRegError>
        where
            T: Clone + Into<M> + Component,
            M: Clone + Into<T> + Any + Send + Sync + Serialize + DeserializeOwned,
    {
        table.register::<NetCompMsg<M>>(transport)?;
        Ok(self.add_system(network_comp_sys::<T, M>))
    }
}

fn network_comp_sys<T, M> (
    server: Option<ResMut<Server>>,
    client: Option<ResMut<Client>>,
    mut q: Query<(&NetEntity, &NetComp<T, M>, &mut T)>,
)
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    if let Some(server) = server {
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
                    if let Err(e) = server.send_spec(&NetCompMsg::<M>::new(net_e.id, comp.clone().into()), spec) {
                        error!("{}", e);
                    }
                }
                NetDirection::ToFrom(to_spec, from_spec) => {
                    // TODO: log overlap
                    // From
                    if let Some(&(_cid, valid_msg)) = msgs.iter().filter(|(cid, msg)| from_spec.matches(*cid) && msg.id == net_e.id).last() {
                        *comp = valid_msg.msg.clone().into();
                    }
                    // To
                    if let Err(e) = server.send_spec(&NetCompMsg::<M>::new(net_e.id, comp.clone().into()), to_spec) {
                        error!("{}", e);
                    }
                }
            }
        }
    } else if let Some(client) = client {
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
                    if let Err(e) = client.send(&NetCompMsg::<M>::new(net_e.id, comp.clone().into())) {
                        error!("{}", e);
                    }
                }
                NetDirection::ToFrom(_, _) => {
                    error!("NetEntity {{ id: {} }} has NetDirection::ToFrom, but this is not allowed on clients.", net_e.id);
                }
            }
        }
    }
}
