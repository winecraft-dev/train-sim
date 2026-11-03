use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, InvalidTrainAxleLoc, InvalidTrainLoc},
    loader::wire,
    loc::{Loc, locator::Locator, traverser::Traverser},
    train::{Car, Train, axle::Axle, store::TrainStore},
};

#[derive(SystemParam)]
pub struct TrainBuilder<'w, 's> {
    store: ResMut<'w, TrainStore>,
    commands: Commands<'w, 's>,
    locator: Locator<'w, 's>,
    cursor: Traverser<'w, 's>,
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

        let mut loc = match self.locator.convert(params.loc) {
            Ok(l) => l,
            Err(e) => return Err(InvalidTrainLoc(Box::new(e))),
        };

        let mut e_cars: Vec<Entity> = Vec::new();
        let mut e_primary_car = Some(e_train);
        let cars = params.cars.unwrap_or(1);
        for car in 0..cars {
            let e_car = self.spawn_car(e_primary_car, &train, &mut loc)?;
            if let Some(_) = e_primary_car {
                e_primary_car = None;
            } else {
                e_cars.push(e_car);
            }
            if car < cars {
                loc = match self
                    .cursor
                    .traverse(loc, loc.dir.vector(train.car_distance))
                {
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
        loc: &mut Loc,
    ) -> Result<Entity, SimError> {
        let main_loc = *loc;
        *loc = match self
            .cursor
            .traverse(main_loc, main_loc.dir.vector(train.axle_distance))
        {
            Ok(l) => l,
            Err(e) => return Err(InvalidTrainAxleLoc(Box::new(e))),
        };

        let e_car = match e_train {
            Some(e_train) => self
                .commands
                .entity(e_train)
                .insert((main_loc, Car, Axle))
                .id(),
            None => self.commands.spawn((main_loc, Car, Axle)).id(),
        };
        let e_rear = self.commands.spawn((*loc, Axle)).id();

        self.commands.entity(e_car).add_child(e_rear);

        Ok(e_car)
    }
}
