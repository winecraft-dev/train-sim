use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    loc::{Dir, FacingLocation, Loc},
    track::{TrackSegment, builder::TrackStore},
};

#[derive(SystemParam)]
pub struct Locator<'w, 's> {
    store: Res<'w, TrackStore>,
    segments: Query<'w, 's, &'static TrackSegment>,
}

impl<'w, 's> Locator<'w, 's> {
    pub fn on_progress(&self, segment: usize, progress: f32, facing: Dir) -> FacingLocation {
        let e_segment = self.store.segments[segment];
        let segment = self.segments.get(e_segment).unwrap();

        let distance = progress * segment.length;
        (
            Loc {
                track: e_segment,
                distance,
            },
            facing,
        )
    }

    pub fn on_end(&self, segment: usize, end: Dir, offset: f32, facing: Dir) -> FacingLocation {
        let e_segment = self.store.segments[segment];
        let segment = self.segments.get(e_segment).unwrap();

        let distance = match end {
            Dir::FacingA => segment.length - offset,
            Dir::FacingB => offset,
        };

        (
            Loc {
                track: e_segment,
                distance,
            },
            facing,
        )
    }
}
