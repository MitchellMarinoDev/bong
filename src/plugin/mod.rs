use bevy::prelude::*;
use crate::plugin::tick::{client_tick, server_tick};

mod tick;
mod net_comp;
mod net;

pub use net::AppExt;
pub use net_comp::NetComp;
pub use net::NetDirection;
pub use net::NetEntity;

pub struct ClientPlugin;

pub struct ServerPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::First, client_tick)
        ;
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::First, server_tick)
        ;
    }
}
