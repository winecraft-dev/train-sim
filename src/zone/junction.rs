use std::collections::HashMap;

use bevy::prelude::*;

pub struct JunctionPlugin;

impl Plugin for JunctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_lookup);
    }
}

#[derive(Resource, Default, DerefMut, Deref)]
pub struct SwitchJunctionLookup(HashMap<Entity, Entity>);

fn init_lookup(mut commands: Commands) {
    commands.insert_resource(SwitchJunctionLookup::default());
}

#[derive(Debug)]
pub enum JunctionVariant {
    Split1_2 {
        signal: Entity,
        switch: Entity,
    },
    Merge2_1 {
        signals: [Entity; 2],
        switch: Entity,
    },
}

#[derive(Component, Debug)]
pub struct Junction {
    pub variant: JunctionVariant,
}
