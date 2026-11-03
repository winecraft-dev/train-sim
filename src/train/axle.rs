use bevy::prelude::*;

use crate::{
    loc::{Dir, Loc, projector::Projector, traverser::Traverser},
    track::TrackNode,
    train::{Car, Derailed, Train},
};

pub struct AxlePlugin;

impl Plugin for AxlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_trains, project_axle_positions).chain());
    }
}

#[derive(Component, Clone, Debug)]
pub struct Axle;

#[derive(Event)]
pub struct AxleMoved {
    pub train: Entity,
    // facing direction refers to the direction the Axle moved, not just which direction it is facing
    pub from: Loc,
    pub to: Loc,
}

#[allow(unused)]
fn speed_direction(old: &mut Loc, new: &mut Loc, speed: f32) {
    let flipped = old.dir != new.dir;
    old.dir = if speed < 0.0 { Dir::A } else { Dir::B };
    new.dir = if !flipped { old.dir } else { old.dir.flip() }
}

fn move_trains(
    traverser: Traverser,
    children: Query<&Children>,
    mut trains: Query<(Entity, &mut Loc, &Train), Without<Derailed>>,
    mut cars: Query<&mut Loc, (With<Car>, Without<Train>)>,
    mut rear_axles: Query<&mut Loc, (With<Axle>, Without<Car>, Without<Train>)>,
) {
    for (e_train, mut train_loc, train) in trains.iter_mut() {
        move_train(
            &traverser,
            children,
            (e_train, &mut train_loc, train),
            &mut cars,
            &mut rear_axles,
        );
    }
}

fn move_train(
    traverser: &Traverser,
    children: Query<&Children>,
    train: (Entity, &mut Loc, &Train),
    cars: &mut Query<&mut Loc, (With<Car>, Without<Train>)>,
    rear_axles: &mut Query<&mut Loc, (With<Axle>, Without<Car>, Without<Train>)>,
) {
    let mut cursor = *train.1;
    let speed = -cursor.dir.vector(train.2.speed);
    cursor = match traverser.traverse(cursor, speed) {
        Ok(l) => l,
        Err(_) => todo!(),
    };

    *train.1 = cursor;
    move_rear_axle(
        traverser,
        &mut cursor,
        train.0,
        train.2,
        children,
        rear_axles,
    );

    let e_cars: Vec<Entity> = match children.get(train.0) {
        Ok(c) => c.iter().filter(|e| cars.contains(*e)).collect(),
        Err(_) => todo!(),
    };

    for e_car in e_cars {
        let Ok(mut loc) = cars.get_mut(e_car) else {
            todo!();
        };

        cursor = match traverser.traverse(cursor, cursor.dir.vector(train.2.car_distance)) {
            Ok(l) => l,
            Err(_) => todo!(),
        };
        *loc = cursor;

        move_rear_axle(traverser, &mut cursor, e_car, train.2, children, rear_axles);
    }
}

fn move_rear_axle(
    traverser: &Traverser,
    cursor: &mut Loc,
    e_car: Entity,
    train: &Train,
    children: Query<&Children>,
    rear_axles: &mut Query<&mut Loc, (With<Axle>, Without<Car>, Without<Train>)>,
) {
    let e_rear = match children.get(e_car) {
        Ok(c) => match c
            .iter()
            .filter(|e| rear_axles.contains(*e))
            .collect::<Vec<Entity>>()
            .first()
        {
            Some(e) => *e,
            None => todo!(),
        },
        Err(_) => todo!(),
    };

    let mut rear_loc = match rear_axles.get_mut(e_rear) {
        Ok(l) => l,
        Err(_) => todo!(),
    };

    *cursor = match traverser.traverse(*cursor, cursor.dir.vector(train.axle_distance)) {
        Ok(l) => l,
        Err(_) => todo!(),
    };

    *rear_loc = *cursor;
}

fn project_axle_positions(
    axles: Query<(Entity, &mut Transform, &Loc), (With<Axle>, Without<TrackNode>)>,
    projector: Projector,
) {
    for (e_axle, mut transform, location) in axles {
        transform.translation = match projector.project(*location) {
            Ok(pos) => pos,
            Err(e) => {
                eprintln!("Problem with axle[{}], skipping: {}", e_axle, e);
                continue;
            }
        };
    }
}
