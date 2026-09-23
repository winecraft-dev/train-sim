use bevy::prelude::*;

use crate::{
    control::TargetClicked,
    landmark::LandmarkPassed,
    signal::control::{Effect, SignalCommand, signal_controlled},
};

pub mod control;

pub struct SignalPlugin;

impl Plugin for SignalPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(train_passed)
            .add_systems(Update, release_trains)
            .add_observer(signal_controlled)
            .add_observer(signal_clicked);
    }
}

#[derive(Component)]
pub struct Signal {
    pub aspect: Aspect,
}

#[derive(Default, Clone, Copy)]
pub enum Aspect {
    Red,
    #[default]
    Green,
}

#[derive(Component)]
pub struct HoldingSignal {
    train: Entity,
}

#[derive(Component)]
pub struct ObservableBound {
    pub signal: Entity,
}

fn train_passed(
    passed: On<LandmarkPassed>,
    mut commands: Commands,
    obv_bounds: Query<&ObservableBound>,
    signals: Query<&Signal>,
) {
    let LandmarkPassed {
        forwards,
        landmark: e_landmark,
        train: e_train,
    } = *passed.event();

    if !forwards {
        return;
    }

    let e_signal = match obv_bounds.get(e_landmark) {
        Ok(s) => s.signal,
        Err(_) => return,
    };

    let signal = match signals.get(e_signal) {
        Ok(s) => s,
        Err(_) => return,
    };

    if let Aspect::Red = signal.aspect {
        commands
            .entity(e_signal)
            .insert(HoldingSignal { train: e_train });
        commands.trigger(SignalCommand {
            effect: Effect::Stop,
            signal: e_signal,
            train: e_train,
        });
    }
}

// TODO: update so SignalControl triggers a check for releasing trains
fn release_trains(
    mut commands: Commands,
    holding_signals: Query<(Entity, &HoldingSignal, &Signal)>,
) {
    for (e_signal, holding, signal) in holding_signals {
        if let Aspect::Green = signal.aspect {
            commands.trigger(SignalCommand {
                effect: Effect::Go,
                signal: e_signal,
                train: holding.train,
            });
            commands.entity(e_signal).remove::<HoldingSignal>();
        }
    }
}

fn signal_clicked(clicked: On<TargetClicked>, mut signals: Query<&mut Signal>) {
    let e_signal = clicked.0;
    let mut signal = match signals.get_mut(e_signal) {
        Ok(s) => s,
        Err(_) => return,
    };
    signal.aspect = match signal.aspect {
        Aspect::Red => Aspect::Green,
        Aspect::Green => Aspect::Red,
    };
}
