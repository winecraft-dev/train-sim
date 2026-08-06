use bevy::prelude::*;

use crate::track::{NodeNeighborsComputed, TrackNode, TrackSegment, TrackUpdated};

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_switches);
    }
}

#[derive(Default, Component)]
pub enum TrackSwitch {
    #[default]
    None,

    Terminus(Entity),
    Track(Entity, Entity),
    Switch {
        control: usize,
        inlet: Entity,
        outlet: [Entity; 2],
    },
    ThreewayTurnout {
        control: usize,
        inlet: Entity,
        outlet: [Entity; 3],
    },
}

impl TrackSwitch {
    pub fn next_segment(&self, inlet: Entity) -> Option<Entity> {
        match self {
            TrackSwitch::None => todo!(),
            TrackSwitch::Terminus(entity) => todo!(),
            TrackSwitch::Track(entity, entity1) => todo!(),
            TrackSwitch::Switch {
                control,
                inlet,
                outlet,
            } => todo!(),
            TrackSwitch::ThreewayTurnout {
                control,
                inlet,
                outlet,
            } => todo!(),
        }
    }
}

pub fn spawn_switches(
    _neighbors_computed: On<NodeNeighborsComputed>,
    mut commands: Commands,
    nodes: Query<(Entity, &TrackNode)>,
) {
}
