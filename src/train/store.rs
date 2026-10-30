use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Resource, Default, DerefMut, Deref, Debug)]
pub struct AxleStore(HashSet<Entity>);

impl AxleStore {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(commands: &mut Commands, mut store: ResMut<Self>) {
        println!("Store: {:?}", store);
        for e_axle in store.iter() {
            println!("e_axle: {:?}", e_axle);
            commands.entity(*e_axle).despawn();
        }
        store.clear();
    }
}
