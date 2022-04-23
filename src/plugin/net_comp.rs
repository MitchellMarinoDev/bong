use std::any::Any;
use std::marker::PhantomData;
use crate::Component;
use crate::plugin::net::NetDirection;
use crate::plugin::net::NetDirection::*;

// TODO: this could be, perhaps split into 2 types. One for up, one for down.
//  This would allow optimizations of parallel up (as they would only need immutable refs).
/// A component that tells `bevy-pigeon` to sync the component `T`
/// which is sent as `M`.
pub struct NetComp<T, M = T>
where
    T: Any + Send + Send + Into<M> + Component,
    M: Any + Send + Send,
{
    dir: NetDirection,
    _pd: PhantomData<(T, M)>
}

impl<T, M> Default for NetComp<T, M>
where
    T: Any + Send + Send + Into<M> + Component,
    M: Any + Send + Send,
{
    fn default() -> Self {
        NetComp {
            dir: Down,
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
