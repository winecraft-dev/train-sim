use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, BrokenSegmentReference, WireConversionFailed},
    loader::wire,
    loc::{Dir, FLoc, Loc},
    track::{TrackSegment, builder::TrackStore},
};

#[derive(SystemParam)]
pub struct Locator<'w, 's> {
    store: Res<'w, TrackStore>,
    segments: Query<'w, 's, &'static TrackSegment>,
}

impl<'w, 's> Locator<'w, 's> {
    pub fn convert(&self, input: wire::FLoc) -> Result<FLoc, SimError> {
        let e_segment = match self.store.segment(input.loc.track) {
            Ok(s) => s,
            Err(e) => return Err(WireConversionFailed(Box::new(e))),
        };
        let Ok(segment) = self.segments.get(e_segment) else {
            return Err(BrokenSegmentReference(e_segment));
        };

        let mut distance = 0.0;
        if let Some(progress) = input.loc.progress {
            distance = progress * segment.length;
        } else if let Some(offset) = input.loc.offset {
            let end = match input.loc.end {
                Some(e) => e,
                None => Dir::default(),
            };
            distance = match end {
                Dir::A => segment.length - offset,
                Dir::B => offset,
            };
        }

        Ok((
            Loc {
                track: e_segment,
                distance: distance,
            },
            input.dir,
        ))
    }

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
}
