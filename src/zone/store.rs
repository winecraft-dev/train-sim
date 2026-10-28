use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct ZoneStore(Vec<Entity>);

impl ZoneStore {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(commands: &mut Commands, mut store: ResMut<Self>) {
        for e_zone in store.iter() {
            commands.entity(*e_zone).despawn();
        }
        store.clear();
    }
}

#[derive(Resource, Default, DerefMut, Deref)]
pub struct SwitchJunctionLookup(HashMap<Entity, Entity>);

impl SwitchJunctionLookup {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(mut lookup: ResMut<Self>) {
        lookup.clear();
    }
}
