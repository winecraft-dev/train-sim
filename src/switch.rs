use bevy::prelude::*;

use crate::track::{NodeNeighborsComputed, TrackNode, TrackSegment, TrackUpdated};

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_switches);
    }
}

#[derive(Default, Debug, Component)]
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
        println!("Switch {:?} finding adjacent segment for [{}]", self, inlet);
        match self {
            TrackSwitch::None => None,
            TrackSwitch::Terminus(_) => None,
            TrackSwitch::Track(a, b) => {
                if *a == inlet {
                    Some(*b)
                } else if *b == inlet {
                    Some(*a)
                } else {
                    println!(
                        "No match found for Inlet[{}] from a[{}] or b[{}]",
                        inlet, a, b
                    );
                    None // should be impossible
                }
            }
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
    segments: Query<&TrackSegment>,
) {
    for (e_origin, origin) in nodes {
        match origin.neighbors.len() {
            0 => println!("Node[{}] with no neighbors!", e_origin),
            1 => {
                let e_terminating_track = origin.neighbors.get(0).unwrap();
                let terminus = TrackSwitch::Terminus(*e_terminating_track);

                commands.entity(e_origin).insert(terminus);
            }
            2 => {
                let e_segment_a = origin.neighbors.get(0).unwrap();
                let e_segment_b = origin.neighbors.get(1).unwrap();
                let track = TrackSwitch::Track(*e_segment_a, *e_segment_b);

                commands.entity(e_origin).insert(track);
            }
            3 => {
                // spawn TrackVariant::Switch
                for e_neighbor in origin.neighbors.iter() {
                    let segment = segments.get(*e_neighbor).unwrap();
                    let angle_from = segment.angle_from(e_origin);

                    // We must group together neighbor segments by the
                    // node angle from Node[origin]

                    todo!();
                }
            }
            x => println!("Node[{}] with length {} not handled", e_origin, x),
        }
    }
}
