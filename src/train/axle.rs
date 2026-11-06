use bevy::prelude::*;

use crate::{
    loc::{Loc, projector::Projector},
    track::TrackNode,
};

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub enum Axle {
    #[default]
    Front,
    Rear,
}

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
