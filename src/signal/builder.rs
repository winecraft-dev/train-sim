use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    landmark::Landmark,
    loc::{Dir, FacingLocation, Loc, cursor::TrackCursor},
    signal::{ObservableBound, Signal},
};

#[derive(SystemParam)]
pub struct SignalBuilder<'w, 's> {
    commands: Commands<'w, 's>,
    cursor: TrackCursor<'w, 's>,
    signals: Query<'w, 's, (Entity, &'static Loc, &'static Dir, &'static Signal)>,
}

impl<'w, 's> SignalBuilder<'w, 's> {
    pub fn new(
        &mut self,
        zone: Entity,
        floc: FacingLocation,
        observable_distance: f32,
    ) -> Option<Entity> {
        let e_signal = self
            .commands
            .spawn((
                Signal {
                    zone,
                    observable_distance,
                },
                floc,
            ))
            .id();
        self.commands.entity(zone).add_child(e_signal);

        Some(e_signal)
    }

    pub fn add_observable_bound(&mut self, signal: Entity) {
        let (e_signal, loc, dir, signal) = self.signals.get(signal).unwrap();

        let obv_loc = match self
            .cursor
            .traverse((*loc, *dir), signal.observable_distance)
        {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Problem adding observable bound: {}", e);
                return;
            }
        };

        let e_obv = self
            .commands
            .spawn((Landmark, ObservableBound { signal: e_signal }, obv_loc))
            .id();
        self.commands
            .entity(e_signal)
            .add_child(e_obv)
            .insert(SignalWithBound);
    }
}

#[derive(Component)]
pub struct SignalWithBound;

pub(super) fn add_observable_bounds(
    signals: Query<Entity, (With<Signal>, Without<SignalWithBound>)>,
    mut builder: SignalBuilder,
) {
    for e_signal in signals {
        builder.add_observable_bound(e_signal);
    }
}
