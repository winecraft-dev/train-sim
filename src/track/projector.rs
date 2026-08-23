use bevy::prelude::*;

use crate::track::{TrackNode, TrackSegment, TrackVariant, error::TrackError, loc::Location};

pub type Projector = Location;

impl Projector {
    pub fn project(
        &self,
        segments: Query<&TrackSegment>,
        nodes: Query<&Transform, With<TrackNode>>,
    ) -> Result<Vec2, TrackError> {
        let segment = match segments.get(self.track) {
            Ok(s) => s,
            Err(_) => return Err(TrackError::BrokenSegmentReference(self.track)),
        };

        let a = match nodes.get(segment.nodes.0) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(TrackError::BrokenSegmentReference(segment.nodes.0)),
        };
        let b = match nodes.get(segment.nodes.1) {
            Ok(transform) => transform.translation.xy(),
            Err(_) => return Err(TrackError::BrokenSegmentReference(segment.nodes.0)),
        };

        match segment.variant {
            TrackVariant::Straight => {
                let projected = a.lerp(b, self.distance / segment.length());
                Ok(projected)
            }
            TrackVariant::Curved {
                center,
                angle,
                radius,
            } => {
                let center = nodes.get(center).unwrap().translation.xy();
                let start_angle = (a - center).to_angle();

                let angle = angle.unwrap();
                let track_radius = radius.unwrap();

                let delta_angle = 0.0.lerp(angle, self.distance / segment.length());
                let theta = start_angle + delta_angle;

                let (sin, cos) = ops::sin_cos(theta);
                let x = cos * track_radius;
                let y = sin * track_radius;
                let position = Vec2::new(x, y) + center;

                Ok(position)
            }
        }
    }
}
