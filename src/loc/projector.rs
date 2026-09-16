use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    loc::{Loc, error::LocError},
    track::{TrackNode, TrackSegment, TrackVariant},
};

#[derive(SystemParam)]
pub struct Projector<'w, 's> {
    segments: Query<'w, 's, &'static TrackSegment>,
    nodes: Query<'w, 's, &'static Transform, With<TrackNode>>,
}

impl<'w, 's> Projector<'w, 's> {
    pub fn project(&self, loc: Loc) -> Result<Vec3, LocError> {
        let segment = match self.segments.get(loc.track) {
            Ok(s) => s,
            Err(_) => return Err(LocError::BrokenSegmentReference(loc.track)),
        };

        let a = match self.nodes.get(segment.nodes.0) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(LocError::BrokenSegmentReference(segment.nodes.0)),
        };
        let b = match self.nodes.get(segment.nodes.1) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(LocError::BrokenSegmentReference(segment.nodes.0)),
        };

        match segment.variant {
            TrackVariant::Straight => {
                let projected = a.lerp(b, loc.distance / segment.length);
                Ok(projected.extend(0.0))
            }
            TrackVariant::Curved {
                center,
                angle,
                radius,
            } => {
                let center = self.nodes.get(center).unwrap().translation.xy();
                let start_angle = (a - center).to_angle();

                let delta_angle = 0.0.lerp(angle, loc.distance / segment.length);
                let theta = start_angle + delta_angle;

                let (sin, cos) = ops::sin_cos(theta);
                let x = cos * radius;
                let y = sin * radius;
                let position = Vec2::new(x, y) + center;

                Ok(position.extend(0.0))
            }
        }
    }
}
