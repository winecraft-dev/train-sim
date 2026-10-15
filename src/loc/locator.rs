use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, BrokenSegmentReference, SegmentNotInStore},
    loc::{Dir, FLoc, Loc},
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
    ) -> Result<FLoc, SimError> {
        let e_segment = self.store.segment(segment)?;
        let Ok(segment) = self.segments.get(e_segment) else {
            return Err(BrokenSegmentReference(e_segment));
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
    ) -> Result<FLoc, SimError> {
        let e_segment = self.store.segment(segment)?;
        let Ok(segment) = self.segments.get(e_segment) else {
            return Err(BrokenSegmentReference(e_segment));
        };

        let distance = match end {
            Dir::A => segment.length - offset,
            Dir::B => offset,
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
