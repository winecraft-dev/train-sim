use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, InvalidTrainAxleLoc, InvalidTrainLoc},
    loader::wire,
    loc::{FLoc, cursor::TrackCursor, locator::Locator},
    train::{Car, Train, axle::Axle, store::AxleStore},
};

#[derive(SystemParam)]
pub struct TrainBuilder<'w, 's> {
    store: ResMut<'w, AxleStore>,
    commands: Commands<'w, 's>,
    locator: Locator<'w, 's>,
    cursor: TrackCursor<'w, 's>,
}

const DEFAULT_TRAIN_SPEED: f32 = 0.3;
const DEFAULT_AXLE_DISTANCE: f32 = 50.0;
const DEFAULT_CAR_DISTANCE: f32 = 30.0;

impl<'w, 's> TrainBuilder<'w, 's> {
    pub fn from_wire(&mut self, params: &wire::Train) -> Result<Entity, SimError> {
        let train = Self::create_train(params);
        let e_train = self
            .commands
            .spawn((GlobalTransform::default(), train.clone()))
            .id();
        self.store.insert(e_train);

        let mut floc = match self.locator.convert(params.floc) {
            Ok(l) => l,
            Err(e) => return Err(InvalidTrainLoc(Box::new(e))),
        };

        let mut e_cars: Vec<Entity> = Vec::new();
        let mut e_primary_car = Some(e_train);
        let cars = params.cars.unwrap_or(1);
        for car in 0..cars {
            let e_car = self.spawn_car(e_primary_car, &train, &mut floc)?;
            if let Some(_) = e_primary_car {
                e_primary_car = None;
            } else {
                e_cars.push(e_car);
            }
            if car < cars {
                floc = match self.cursor.traverse(floc, train.car_distance) {
                    Ok(l) => l,
                    Err(e) => return Err(InvalidTrainAxleLoc(Box::new(e))),
                };
            }
        }

        self.commands.entity(e_train).add_children(&e_cars);

        Ok(e_train)
    }

    fn create_train(params: &wire::Train) -> Train {
        let speed = params.speed.unwrap_or(DEFAULT_TRAIN_SPEED);
        let car_distance = params.car_distance.unwrap_or(DEFAULT_CAR_DISTANCE);
        let axle_distance = params.axle_distance.unwrap_or(DEFAULT_AXLE_DISTANCE);

        Train {
            speed,
            car_distance,
            axle_distance,
        }
    }

    fn spawn_car(
        &mut self,
        e_train: Option<Entity>, // Some if this is the 1st car
        train: &Train,
        floc: &mut FLoc,
    ) -> Result<Entity, SimError> {
        let main_loc = *floc;
        *floc = match self.cursor.traverse(main_loc, train.axle_distance) {
            Ok(l) => l,
            Err(e) => return Err(InvalidTrainAxleLoc(Box::new(e))),
        };

        let e_car = match e_train {
            Some(e_train) => self
                .commands
                .entity(e_train)
                .insert((main_loc, Car, Axle))
                .id(),
            None => {
                let e_car = self.commands.spawn((main_loc, Car, Axle)).id();
                self.store.insert(e_car);
                e_car
            }
        };
        let e_rear = self.commands.spawn((*floc, Axle)).id();
        self.store.insert(e_rear);

        self.commands.entity(e_car).add_child(e_rear);

        Ok(e_car)
    }
}
