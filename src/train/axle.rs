use bevy::prelude::*;

use crate::{
    loc::{Dir, Loc, projector::Projector},
    track::TrackNode,
};

#[derive(Component)]
pub struct Axle;

pub fn project_axle_positions(
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

#[derive(Event, Debug)]
pub struct AxleMoved {
    pub train: Entity,
    // facing direction refers to the direction the Axle moved, not just which direction it is facing
    pub from: Loc,
    pub to: Loc,
}

pub fn speed_direction(old: &mut Loc, new: &mut Loc, speed: f32) {
    let flipped = old.dir != new.dir;
    old.dir = if speed < 0.0 { Dir::A } else { Dir::B };
    new.dir = if !flipped { old.dir } else { old.dir.flip() }
}
