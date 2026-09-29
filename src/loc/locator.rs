use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    loc::{Dir, FacingLocation, Loc, error::LocError},
    track::{TrackSegment, builder::TrackStore},
};

#[derive(SystemParam)]
pub struct Locator<'w, 's> {
    store: Res<'w, TrackStore>,
    segments: Query<'w, 's, &'static TrackSegment>,
}

impl<'w, 's> Locator<'w, 's> {
    pub fn on_progress(
        &self,
        segment: usize,
        progress: f32,
        facing: Dir,
    ) -> Result<FacingLocation, LocError> {
        let Some(e_segment) = self.store.segment(segment) else {
            return Err(LocError::SegmentNotInStore(segment));
        };
        let Ok(segment) = self.segments.get(e_segment) else {
            return Err(LocError::BrokenSegmentReference(e_segment));
        };

        let distance = progress * segment.length;
        Ok((
            Loc {
                track: e_segment,
                distance,
            },
            facing,
        ))
    }

    pub fn on_end(
        &self,
        segment: usize,
        end: Dir,
        offset: f32,
        facing: Dir,
    ) -> Result<FacingLocation, LocError> {
        let Some(e_segment) = self.store.segment(segment) else {
            return Err(LocError::SegmentNotInStore(segment));
        };
        let Ok(segment) = self.segments.get(e_segment) else {
            return Err(LocError::BrokenSegmentReference(e_segment));
        };

        let distance = match end {
            Dir::ToA => segment.length - offset,
            Dir::ToB => offset,
        };

        Ok((
            Loc {
                track: e_segment,
                distance,
            },
            facing,
        ))
    }
}
