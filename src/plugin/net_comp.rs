use std::any::Any;
use std::marker::PhantomData;
use carrier_pigeon::net::CIdSpec::All;
use crate::Component;
use crate::plugin::net::NetDirection;
use crate::plugin::net::NetDirection::*;

/// A component that tells `bevy-pigeon` to sync the component `T`
/// which is sent as `M`.
#[derive(Component)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NetComp<T, M = T>
where
    T: Any + Send + Send + Into<M> + Component,
    M: Any + Send + Send,
{
    pub dir: NetDirection,
    _pd: PhantomData<(T, M)>
}

impl<T, M> Default for NetComp<T, M>
where
    T: Any + Send + Send + Into<M> + Component,
    M: Any + Send + Send,
{
    fn default() -> Self {
        NetComp {
            dir: From(All),
            _pd: PhantomData::default(),
        }
    }
}

impl<T, M> NetComp<T, M>
    where
        T: Any + Send + Send + Into<M> + Component,
        M: Any + Send + Send,
{
    fn new(dir: NetDirection) -> Self {
        NetComp {
            dir,
            _pd: PhantomData::default(),
        }
    }
}
