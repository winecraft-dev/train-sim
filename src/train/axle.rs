use bevy::prelude::*;

use crate::{
    error::SimError::{self, InvalidTrainAxleLoc, NoChildren, NoRearAxle},
    loc::{Dir, Loc, projector::Projector, traverser::Traverser},
    track::TrackNode,
    train::{Car, Derailed, Train, TrainDerailed},
};

pub struct AxlePlugin;

impl Plugin for AxlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_train_speeds, project_axle_positions).chain());
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

fn speed_direction(old: &mut Loc, new: &mut Loc, speed: f32) {
    let flipped = old.dir != new.dir;
    old.dir = if speed < 0.0 { Dir::A } else { Dir::B };
    new.dir = if !flipped { old.dir } else { old.dir.flip() }
}

fn apply_train_speeds(
    mut commands: Commands,
    mut trains: Query<(Entity, &mut Loc, &Train), Without<Derailed>>,
    cars: Query<&mut Loc, (With<Car>, Without<Train>)>,
    mut rear_axles: Query<&mut Loc, (With<Axle>, Without<Car>, Without<Train>)>,
    children: Query<&Children>,
    traverser: Traverser,
) {
    for (e_train, mut train_loc, train) in trains.iter_mut() {
        let Ok(c_train) = children.get(e_train) else {
            continue;
        };

        let mut cursor = *train_loc;
        let speed = -cursor.dir.vector(train.speed);
        cursor = match traverser.traverse(cursor, speed) {
            Ok(l) => l,
            Err(_) => {
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };
        apply_rear_axle(
            &traverser,
            &mut cursor,
            e_train,
            children,
            &mut rear_axles,
            train,
        );

        let e_cars: Vec<Entity> = c_train.iter().filter(|e| cars.get(*e).is_ok()).collect();
        for e_car in c_train {
            let Some(mut car_loc) = cars.get(*e_car).ok() else {
                continue;
            };
        }
    }
}

fn apply_rear_axle(
    traverser: &Traverser,
    cursor: &mut Loc,
    e_car: Entity,
    children: Query<&Children>,
    rear_axles: &mut Query<&mut Loc, (With<Axle>, Without<Car>, Without<Train>)>,
    train: &Train,
) -> Result<(), SimError> {
    let Ok(c_car) = children.get(e_car) else {
        return Err(NoChildren(e_car));
    };
    let e_rear: Vec<Entity> = c_car.iter().filter(|e| rear_axles.contains(*e)).collect();
    if e_rear.len() <= 1 {
        return Err(NoRearAxle(e_car));
    }
    let mut rear_loc = rear_axles.get_mut(e_rear[0]).unwrap(); // already checked

    *cursor = match traverser.traverse(*cursor, train.axle_distance) {
        Ok(l) => l,
        Err(e) => return Err(InvalidTrainAxleLoc(Box::new(e))),
    };

    *rear_loc = *cursor;

    Ok(())
}
// let mut old_train_floc = (*main_loc, *main_dir);
// let speed = -old_train_floc.1.vector(train.speed);
// let children: Vec<Entity> = match children.get(e_train) {
//     Ok(c) => c.to_vec(),
//     Err(_) => Vec::default(),
// };

// let mut new_train_floc = match cursor.traverse(old_train_floc, speed) {
//     Ok(l) => l,
//     Err(_) => {
//         commands.trigger(TrainDerailed(e_train));
//         continue;
//     }
// };

// *main_loc = new_train_floc.0;
// *main_dir = new_train_floc.1;

// let mut axle_floc = new_train_floc;
// for e_axle in children {
//     let (next_axle, mut next_loc, mut next_dir) = match axles.get_mut(e_axle) {
//         Ok(a) => a,
//         Err(_) => {
//             eprintln!("Problem getting child axle[{}]", e_axle);
//             continue;
//         }
//     };

//     let offset = axle_floc.1.vector(next_axle.offset);
//     axle_floc = match cursor.traverse(axle_floc, offset) {
//         Ok(loc) => loc,
//         Err(_) => {
//             commands.trigger(TrainDerailed(e_train));
//             return;
//         }
//     };
//     *next_loc = axle_floc.0;
//     *next_dir = axle_floc.1;
// }

// // probably a bug here... maybe it will get exposed when a train
// // traverses beyond a single segment. Edge case that might turn
// // up later.
// speed_direction(&mut old_train_floc, &mut new_train_floc, speed);

// commands.trigger(AxleMoved {
//     train: e_train,
//     from: old_train_floc,
//     to: new_train_floc,
// });

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
