use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, InvalidTrainLoc},
    loader::wire,
    loc::{cursor::TrackCursor, locator::Locator},
    train::{Train, axle::Axle, store::AxleStore},
};

#[derive(SystemParam)]
pub struct TrainBuilder<'w, 's> {
    store: ResMut<'w, AxleStore>,
    commands: Commands<'w, 's>,
    locator: Locator<'w, 's>,
    cursor: TrackCursor<'w, 's>,
}

const DEFAULT_TRAIN_SPEED: f32 = 0.3;
pub const DEFAULT_AXLE_DISTANCE: f32 = 50.0;
// const DEFAULT_CAR_DISTANCE: f32 = 30.0;

impl<'w, 's> TrainBuilder<'w, 's> {
    pub fn from_wire(&mut self, train: &wire::Train) -> Result<Entity, SimError> {
        let speed = match train.speed {
            Some(s) => s,
            None => DEFAULT_TRAIN_SPEED,
        };
        let e_train = self
            .commands
            .spawn((GlobalTransform::default(), Train { speed }))
            .id();

        let main_floc = match self.locator.convert(train.floc) {
            Ok(l) => l,
            Err(e) => return Err(InvalidTrainLoc(Box::new(e))),
        };

        let rear_offset = main_floc.1.vector(DEFAULT_AXLE_DISTANCE);
        let rear_floc = match self.cursor.traverse(main_floc, rear_offset) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        let e_rear = self
            .commands
            .spawn((
                rear_floc,
                Axle {
                    offset: DEFAULT_AXLE_DISTANCE,
                },
            ))
            .id();
        self.commands
            .entity(e_train)
            .insert((main_floc, Axle { offset: 0.0 }))
            .add_child(e_rear);

        self.store.push(e_rear);
        self.store.push(e_train);

        Ok(e_train)
    }
}
