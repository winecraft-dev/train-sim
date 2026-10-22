use bevy::prelude::*;

#[derive(Resource, Default, DerefMut, Deref, Debug)]
pub struct AxleStore(Vec<Entity>);

impl AxleStore {
    pub fn init(mut commands: Commands) {
        commands.insert_resource(Self::default());
    }

    pub fn reset(commands: &mut Commands, mut store: ResMut<Self>) {
        for e_axle in store.iter() {
            commands.entity(*e_axle).despawn();
        }
        store.clear();
    }
}
