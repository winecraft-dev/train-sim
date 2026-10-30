use bevy::prelude::*;

use crate::{
    loc::{Dir, FLoc, Loc, cursor::TrackCursor, projector::Projector},
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
    pub from: FLoc,
    pub to: FLoc,
}

fn speed_direction(old: &mut FLoc, new: &mut FLoc, speed: f32) {
    let flipped = old.1 != new.1;
    old.1 = if speed < 0.0 { Dir::A } else { Dir::B };
    new.1 = if !flipped { old.1 } else { old.1.flip() }
}

fn apply_train_speeds(
    mut commands: Commands,
    mut trains: Query<(Entity, &mut Loc, &mut Dir, &Train), Without<Derailed>>,
    cars: Query<(&mut Loc, &mut Dir), (With<Car>, Without<Train>)>,
    rear_axles: Query<(&mut Loc, &mut Dir), (With<Axle>, Without<Car>, Without<Train>)>,
    children: Query<&Children>,
    cursor: TrackCursor,
) {
    for (e_train, mut train_loc, mut train_dir, train) in trains.iter_mut() {
        let Ok(c_train) = children.get(e_train) else {
            continue;
        };
        let mut train_floc = (*train_loc, *train_dir);
        let speed = -train_floc.1.vector(train.speed);
        train_floc = match cursor.traverse(train_floc, speed) {
            Ok(l) => l,
            Err(_) => {
                commands.trigger(TrainDerailed(e_train));
                continue;
            }
        };

        *train_loc = train_floc.0;
        *train_dir = train_floc.1;

        for e_car in c_train {
            let Some((mut car_loc, mut car_dir)) = cars.get(*e_car).ok() else {
                continue;
            };
        }
    }
}

fn apply_car_speeds(
    car_floc: &mut FLoc,
    rear_floc: &mut FLoc,
    children: Query<&Children>,
    cursor: TrackCursor,
) {
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
