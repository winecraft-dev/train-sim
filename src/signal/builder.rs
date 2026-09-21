use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    control::ClickTarget,
    landmark::Landmark,
    loc::{FacingLocation, cursor::TrackCursor},
    signal::{Aspect, ObservableBound, Signal},
};

#[derive(SystemParam)]
pub struct SignalBuilder<'w, 's> {
    commands: Commands<'w, 's>,
    cursor: TrackCursor<'w, 's>,
}

impl<'w, 's> SignalBuilder<'w, 's> {
    // maybe we can move creating obsv bound into here so it's not split in two
    pub fn new(&mut self, floc: FacingLocation, observable_distance: f32) -> Entity {
        let e_signal = self
            .commands
            .spawn((
                Signal {
                    aspect: Aspect::default(),
                },
                ClickTarget,
                floc,
            ))
            .id();

        let obv_loc = self
            .cursor
            .traverse((floc.0, floc.1), observable_distance)
            .unwrap();
        let e_obv = self
            .commands
            .spawn((ObservableBound { signal: e_signal }, Landmark, obv_loc))
            .id();

        self.commands.entity(e_signal).add_child(e_obv);

        e_signal
    }
}
