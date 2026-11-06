use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{
        self, BrokenRearAxleReference, BrokenTrainCarReference, InvalidRearAxleLoc,
        InvalidTrainLoc, NoCars, NoRearAxle,
    },
    landmark::Landmark,
    loc::{Dir, Loc, traverser::Traverser},
    train::{Car, Train, TrainMoved, axle::Axle},
};

#[derive(SystemParam)]
pub struct TrainMover<'w, 's> {
    traverser: Traverser<'w, 's>,
    children: Query<'w, 's, &'static Children>,
    cars: Query<'w, 's, &'static mut Loc, (With<Car>, Without<Train>, Without<Landmark>)>,
    rear_axles: Query<
        'w,
        's,
        &'static mut Loc,
        (With<Axle>, Without<Car>, Without<Train>, Without<Landmark>),
    >,
}

impl<'w, 's> TrainMover<'w, 's> {
    pub fn move_train(&mut self, train: (Entity, Loc, &Train)) -> Result<TrainMoved, SimError> {
        let (e_train, mut old_train_loc, train) = train;

        let mut cursor = old_train_loc;
        let speed = -cursor.dir.vector(train.speed);
        cursor = match self.traverser.traverse(cursor, speed) {
            Ok(l) => l,
            Err(e) => return Err(InvalidTrainLoc(Box::new(e))),
        };

        let mut new_train_loc = cursor;
        let (mut old_rear_loc, mut new_rear_loc) = self.move_rear_axle(cursor, e_train, train)?;
        cursor = new_rear_loc;

        let e_cars: Vec<Entity> = match self.children.get(e_train) {
            Ok(c) => c.iter().filter(|e| self.cars.contains(*e)).collect(),
            Err(_) => return Err(NoCars(e_train)),
        };

        for e_car in e_cars {
            let Ok(mut loc) = self.cars.get_mut(e_car) else {
                return Err(BrokenTrainCarReference(e_car));
            };

            cursor = match self
                .traverser
                .traverse(cursor, cursor.dir.vector(train.car_distance))
            {
                Ok(l) => l,
                Err(e) => return Err(InvalidTrainLoc(Box::new(e))),
            };
            *loc = cursor;

            (old_rear_loc, new_rear_loc) = self.move_rear_axle(cursor, e_car, train)?;
            cursor = new_rear_loc;
        }

        // cursor is now the position of the last rear_axle
        Ok(TrainMoved {
            front: (old_train_loc, new_train_loc),
            rear: (old_rear_loc, new_rear_loc),
        })
    }

    fn move_rear_axle(
        &mut self,
        mut cursor: Loc,
        e_car: Entity,
        train: &Train,
    ) -> Result<(Loc, Loc), SimError> {
        let e_rear = match self.children.get(e_car) {
            Ok(c) => match c
                .iter()
                .filter(|e| self.rear_axles.contains(*e))
                .collect::<Vec<Entity>>()
                .first()
            {
                Some(e) => *e,
                None => return Err(NoRearAxle(e_car)),
            },
            Err(_) => return Err(NoRearAxle(e_car)),
        };

        let mut rear_loc = match self.rear_axles.get_mut(e_rear) {
            Ok(l) => l,
            Err(_) => return Err(BrokenRearAxleReference(e_rear)),
        };
        let old_loc = *rear_loc;

        cursor = match self
            .traverser
            .traverse(cursor, cursor.dir.vector(train.axle_distance))
        {
            Ok(l) => l,
            Err(e) => return Err(InvalidRearAxleLoc(Box::new(e))),
        };

        *rear_loc = cursor;

        Ok((old_loc, cursor))
    }
}
