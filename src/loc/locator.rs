use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    error::SimError::{self, BrokenSegmentReference, WireConversionFailed},
    loader::wire,
    loc::{Dir, Loc},
    track::{TrackSegment, store::SegmentStore},
};

#[derive(SystemParam)]
pub struct Locator<'w, 's> {
    segment_store: Res<'w, SegmentStore>,
    segments: Query<'w, 's, &'static TrackSegment>,
}

impl<'w, 's> Locator<'w, 's> {
    pub fn convert(&self, input: wire::Loc) -> Result<Loc, SimError> {
        let e_segment = match self.segment_store.segment(input.track) {
            Ok(s) => s,
            Err(e) => return Err(WireConversionFailed(Box::new(e))),
        };
        let Ok(segment) = self.segments.get(e_segment) else {
            return Err(BrokenSegmentReference(e_segment));
        };

        let mut distance = 0.0;
        if let Some(progress) = input.progress {
            distance = progress * segment.length;
        } else if let Some(offset) = input.offset {
            let end = match input.end {
                Some(e) => e,
                None => Dir::default(),
            };
            distance = match end {
                Dir::A => segment.length - offset,
                Dir::B => offset,
            };
        }

        Ok(Loc {
            track: e_segment,
            distance: distance,
            dir: input.dir,
        })
    }
}
