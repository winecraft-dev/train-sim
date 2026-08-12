use std::f32::consts::PI;

use bevy::{ecs::relationship::RelationshipSourceCollection, input::keyboard::Key, prelude::*};

use crate::track::{NodeNeighborsComputed, TrackNode, TrackSegment};

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_switches)
            .add_systems(Update, bump_switches);
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
    pub fn next_segment(&self, current: Entity) -> Option<Entity> {
        match *self {
            TrackSwitch::None => None,
            TrackSwitch::Terminus(_) => None,
            TrackSwitch::Track(a, b) => {
                if a == current {
                    Some(b)
                } else if b == current {
                    Some(a)
                } else {
                    None
                }
            }
            TrackSwitch::Switch {
                control,
                inlet,
                outlet,
            } => {
                if inlet == current {
                    Some(outlet[control])
                } else {
                    Some(inlet)
                }
            }
            TrackSwitch::ThreewayTurnout {
                control,
                inlet,
                outlet,
            } => {
                if inlet == current {
                    Some(outlet[control])
                } else {
                    Some(inlet)
                }
            }
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
                let (inlet, outlet) = split_ends(e_origin, origin, segments);
                let switch = TrackSwitch::Switch {
                    control: 0,
                    inlet,
                    outlet: *outlet.as_array().unwrap(),
                };
                commands.entity(e_origin).insert(switch);
            }
            4 => {
                let (inlet, outlet) = split_ends(e_origin, origin, segments);
                let turnout = TrackSwitch::ThreewayTurnout {
                    control: 0,
                    inlet,
                    outlet: *outlet.as_array().unwrap(),
                };
                commands.entity(e_origin).insert(turnout);
            }
            _ => unreachable!(),
        }
    }
}

fn split_ends(
    e_origin: Entity,
    origin: &TrackNode,
    segments: Query<&TrackSegment>,
) -> (Entity, Vec<Entity>) {
    let mut end: Option<f32> = None;
    let mut groups: (Vec<Entity>, Vec<Entity>) = (Vec::default(), Vec::default());

    for e_neighbor in origin.neighbors.iter() {
        let segment = segments.get(e_neighbor).unwrap();
        let angle_from = segment.angle_from(e_origin).unwrap();

        match end {
            None => {
                end = Some(angle_from);
                groups.0.add(e_neighbor);
            }
            Some(end_angle) => {
                let diff = angle_from - end_angle;
                if diff > PI / -2.0 && diff < PI / 2.0 {
                    groups.0.add(e_neighbor);
                } else {
                    groups.1.add(e_neighbor);
                }
            }
        }
    }
    if groups.0.len() == 1 {
        (*groups.0.first().unwrap(), groups.1)
    } else {
        (*groups.1.first().unwrap(), groups.0)
    }
}

fn bump_switches(switches: Query<&mut TrackSwitch>, key_input: Res<ButtonInput<Key>>) {
    if key_input.just_pressed(Key::Space) {
        for mut switch in switches {
            match &mut *switch {
                TrackSwitch::Switch {
                    control,
                    inlet: _,
                    outlet: _,
                } => *control = (*control + 1) % 2,
                TrackSwitch::ThreewayTurnout {
                    control,
                    inlet: _,
                    outlet: _,
                } => *control = (*control + 1) % 3,
                _ => continue,
            };
            println!("Switch changed: {:?}", switch);
        }
    }
}
