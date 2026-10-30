use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Resource, Default, DerefMut, Deref, Debug)]
pub struct TrainStore(HashSet<Entity>);

impl TrainStore {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(commands: &mut Commands, mut store: ResMut<Self>) {
        println!("Store: {:?}", store);
        for e_train in store.iter() {
            println!("e_train: {:?}", e_train);
            commands.entity(*e_train).despawn();
        }
        store.clear();
    }
}
