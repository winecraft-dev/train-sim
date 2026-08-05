use bevy::prelude::*;

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Default, Component)]
pub enum TrackSwitch {
    #[default]
    None,

    Terminus(Entity),
    Track(Entity, Entity),
    Switch {
        inlet: Entity,
        outlet: [Entity; 2],
    },
    ThreewayTurnout {
        control: usize,
        inlet: Entity,
        outlet: [Entity; 3],
    },
    Crossover {
        inlet: (Entity, Entity),
        outlet: (Entity, Entity),
    },
}
