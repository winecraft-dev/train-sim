use bevy::prelude::*;

use crate::signal::{Aspect, Signal};

#[derive(Event)]
pub struct SignalControl {
    pub signal: Entity,
    pub aspect: Aspect,
}

pub fn signal_controlled(control: On<SignalControl>, mut signals: Query<&mut Signal>) {
    let SignalControl {
        signal: e_signal,
        aspect,
    } = control.event();

    let mut signal = match signals.get_mut(*e_signal) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Skipping unfound signal[{}]: {}", e_signal, e);
            return;
        }
    };

    signal.aspect = *aspect;
}

#[derive(Event)]
pub struct SignalCommand {
    pub effect: Effect,

    pub signal: Entity, // use this eventually for junction decisioning
    pub train: Entity,
}

#[derive(Event, Clone, Copy)]
pub enum Effect {
    Stop,
    Go,
}
