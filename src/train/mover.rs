use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{
        self, BrokenRearAxleReference, BrokenTrainCarReference, InvalidRearAxleLoc,
        InvalidTrainLoc, NoCars, NoRearAxle,
    },
    landmark::Landmark,
    loc::{Dir, Loc, traverser::Traverser},
    train::{Car, Train, axle::Axle},
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
    pub fn calculate_movement(&self, train: (Loc, &Train)) -> Result<CalculatedMove, SimError> {
        let (train_loc, train) = train;

        let mut cursor = train_loc;
        let mut movement: CalculatedMove = Vec::new();

        let speed = -cursor.dir.vector(train.speed);
        cursor = match self.traverser.traverse(cursor, speed) {
            Ok(l) => l,
            Err(e) => return Err(InvalidTrainLoc(Box::new(e))),
        };

        for car in 0..train.cars {
            let front_axle = cursor;
            cursor = match self
                .traverser
                .traverse(cursor, cursor.dir.vector(train.axle_distance))
            {
                Ok(l) => l,
                Err(e) => return Err(InvalidRearAxleLoc(Box::new(e))),
            };
            let rear_axle = cursor;
            movement.push((front_axle, rear_axle));
            if car < train.cars {
                cursor = match self
                    .traverser
                    .traverse(cursor, cursor.dir.vector(train.car_distance))
                {
                    Ok(l) => l,
                    Err(e) => return Err(InvalidTrainLoc(Box::new(e))),
                }
            }
        }

        Ok(movement)
    }

    pub fn move_train(
        &mut self,
        train: (Entity, &mut Loc, &Train),
        calculated: CalculatedMove,
    ) -> Result<TrainMoved, SimError> {
        let (e_train, train_loc, train) = train;

        if calculated.len() != train.cars {
            todo!();
        }
        let mut calculated = calculated.iter();
        let mut movement = TrainMoved::new(e_train);

        let (front, rear) = calculated.next().unwrap();
        movement.move_axle(e_train, train_loc, *front);
        movement.add(self.move_rear_axle(e_train, *rear)?);

        let e_cars: Vec<Entity> = match self.children.get(e_train) {
            Ok(c) => c.iter().filter(|e| self.cars.contains(*e)).collect(),
            Err(_) => return Err(NoCars(e_train)),
        };

        for e_car in e_cars {
            let (front, rear) = calculated.next().unwrap();
            let Ok(mut loc) = self.cars.get_mut(e_car) else {
                return Err(BrokenTrainCarReference(e_car));
            };
            movement.move_axle(e_car, &mut loc, *front);
            movement.add(self.move_rear_axle(e_car, *rear)?);
        }

        Ok(movement)
    }

    fn move_rear_axle(&mut self, e_car: Entity, loc: Loc) -> Result<AxleMoved, SimError> {
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
        *rear_loc = loc;

        Ok(AxleMoved {
            axle: e_rear,
            from: old_loc,
            to: loc,
        })
    }
}

pub type CalculatedMove = Vec<(Loc, Loc)>;

#[derive(Debug)]
pub struct TrainMoved {
    train: Entity,
    movement: Vec<AxleMoved>,
}

impl TrainMoved {
    pub fn new(train: Entity) -> Self {
        Self {
            train,
            movement: Vec::new(),
        }
    }

    pub fn move_axle(&mut self, axle: Entity, original: &mut Loc, new: Loc) {
        let old = *original;
        *original = new;
        self.movement.push(AxleMoved {
            axle,
            from: old,
            to: new,
        });
    }

    pub fn add(&mut self, moved: AxleMoved) {
        self.movement.push(moved);
    }

    pub fn speed_direction(&mut self, speed: f32) {
        for axle in self.movement.iter_mut() {
            axle.speed_direction(speed);
        }
    }
}

#[derive(Debug)]
pub struct AxleMoved {
    axle: Entity,
    from: Loc,
    to: Loc,
}

impl AxleMoved {
    fn speed_direction(&mut self, speed: f32) {
        let flipped = self.from.dir != self.to.dir;
        self.from.dir = if speed < 0.0 { Dir::A } else { Dir::B };
        self.to.dir = if !flipped {
            self.from.dir
        } else {
            self.from.dir.flip()
        };
    }
}
