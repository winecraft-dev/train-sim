use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError,
    loc::{Loc, traverser::Traverser},
    train::{Car, Train, axle::Axle},
};

#[derive(SystemParam)]
pub struct TrainMover<'w, 's> {
    traverser: Traverser<'w, 's>,
    children: Query<'w, 's, &'static Children>,
    cars: Query<'w, 's, &'static mut Loc, (With<Car>, Without<Train>)>,
    rear_axles: Query<'w, 's, &'static mut Loc, (With<Axle>, Without<Car>, Without<Train>)>,
}

impl<'w, 's> TrainMover<'w, 's> {
    pub fn move_train(&mut self, mut train: (Entity, Loc, &Train)) -> Result<Loc, SimError> {
        let mut cursor = train.1;
        let speed = -cursor.dir.vector(train.2.speed);
        cursor = match self.traverser.traverse(cursor, speed) {
            Ok(l) => l,
            Err(_) => todo!(),
        };

        train.1 = cursor;
        cursor = self.move_rear_axle(cursor, train.0, train.2)?;

        let e_cars: Vec<Entity> = match self.children.get(train.0) {
            Ok(c) => c.iter().filter(|e| self.cars.contains(*e)).collect(),
            Err(_) => todo!(),
        };

        for e_car in e_cars {
            let Ok(mut loc) = self.cars.get_mut(e_car) else {
                todo!();
            };

            cursor = match self
                .traverser
                .traverse(cursor, cursor.dir.vector(train.2.car_distance))
            {
                Ok(l) => l,
                Err(_) => todo!(),
            };
            *loc = cursor;

            cursor = self.move_rear_axle(cursor, e_car, train.2)?;
        }

        Ok(train.1)
    }

    fn move_rear_axle(
        &mut self,
        mut cursor: Loc,
        e_car: Entity,
        train: &Train,
    ) -> Result<Loc, SimError> {
        let e_rear = match self.children.get(e_car) {
            Ok(c) => match c
                .iter()
                .filter(|e| self.rear_axles.contains(*e))
                .collect::<Vec<Entity>>()
                .first()
            {
                Some(e) => *e,
                None => todo!(),
            },
            Err(_) => todo!(),
        };

        let mut rear_loc = match self.rear_axles.get_mut(e_rear) {
            Ok(l) => l,
            Err(_) => todo!(),
        };

        cursor = match self
            .traverser
            .traverse(cursor, cursor.dir.vector(train.axle_distance))
        {
            Ok(l) => l,
            Err(_) => todo!(),
        };

        *rear_loc = cursor;

        Ok(cursor)
    }
}
