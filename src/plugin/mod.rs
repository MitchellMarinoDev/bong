use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

pub struct PigeonPlugins;
pub struct SendRecvPlugin;

impl PluginGroup for PigeonPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(SendRecvPlugin)
        ;
    }
}