use crate::plugin::net::NetDirection;
use crate::plugin::net::NetDirection::*;
use crate::Component;
use carrier_pigeon::net::CIdSpec::All;
use std::any::Any;
use std::marker::PhantomData;

/// A component that tells `bevy-pigeon` to sync the component `T`
/// which is sent as `M`.
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NetComp<T, M = T>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    // TODO: Add option for changed only.
    pub dir: NetDirection,
    _pd: PhantomData<(T, M)>,
}

impl<T, M> Default for NetComp<T, M>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    fn default() -> Self {
        NetComp {
            dir: From(All),
            _pd: PhantomData,
        }
    }
}

impl<T, M> NetComp<T, M>
where
    T: Clone + Into<M> + Component,
    M: Clone + Into<T> + Any + Send + Sync,
{
    pub fn new(dir: NetDirection) -> Self {
        NetComp {
            dir,
            _pd: PhantomData::default(),
        }
    }
}
