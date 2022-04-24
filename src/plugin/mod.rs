use crate::plugin::tick::{client_tick, server_tick};
use bevy::prelude::*;

mod net;
mod net_comp;
mod tick;

pub use net::AppExt;
pub use net::NetDirection;
pub use net::NetEntity;
pub use net_comp::NetComp;

pub struct ClientPlugin;

pub struct ServerPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, client_tick);
    }
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::First, server_tick);
    }
}
