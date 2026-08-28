use std::f32::consts::PI;

use bevy::{ecs::relationship::RelationshipSourceCollection, prelude::*};

use crate::{
    control::{ClickTarget, TargetClicked},
    track::{NodeNeighborsComputed, SwitchesSpawned, TrackNode, TrackSegment},
};

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_switches)
            .add_observer(switch_clicked);
    }
}

#[derive(Default, Debug, Component)]
pub enum TrackSwitch {
    #[default]
    None,

    #[allow(dead_code)]
    Terminus(Entity), // use this TrackSegment later down the line...
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
            TrackSwitch::Track(a, b) => {
                if a == current {
                    return Some(b);
                } else if b == current {
                    return Some(a);
                }
            }
            TrackSwitch::Switch {
                control,
                inlet,
                outlet,
            } => {
                if inlet == current {
                    return Some(outlet[control]);
                } else {
                    return Some(inlet);
                }
            }
            TrackSwitch::ThreewayTurnout {
                control,
                inlet,
                outlet,
            } => {
                if inlet == current {
                    return Some(outlet[control]);
                } else {
                    return Some(inlet);
                }
            }
            _ => {}
        };
        None
    }
}

pub fn spawn_switches(
    _neighbors_computed: On<NodeNeighborsComputed>,
    mut commands: Commands,
    nodes: Query<(Entity, &TrackNode)>,
    segments: Query<&TrackSegment>,
) {
    for (e_origin, origin) in nodes {
        let switch = match origin.neighbors.len() {
            0 => {
                println!("Node[{}] with no neighbors!", e_origin);
                continue;
            }
            1 => {
                let e_terminating_track = origin.neighbors.get(0).unwrap();
                TrackSwitch::Terminus(*e_terminating_track)
            }
            2 => {
                let e_segment_a = origin.neighbors.get(0).unwrap();
                let e_segment_b = origin.neighbors.get(1).unwrap();
                TrackSwitch::Track(*e_segment_a, *e_segment_b)
            }
            3 => {
                let (inlet, outlet) = split_ports::<2>(e_origin, origin, segments);
                TrackSwitch::Switch {
                    control: 0,
                    inlet,
                    outlet: outlet,
                }
            }
            4 => {
                let (inlet, outlet) = split_ports::<3>(e_origin, origin, segments);
                TrackSwitch::ThreewayTurnout {
                    control: 0,
                    inlet,
                    outlet: outlet,
                }
            }
            _ => unreachable!(),
        };
        commands.entity(e_origin).insert((switch, ClickTarget));
    }
    println!("Done Spawning Switches");
    commands.trigger(SwitchesSpawned);
}

fn split_ports<const OUTLET_N: usize>(
    e_origin: Entity,
    origin: &TrackNode,
    segments: Query<&TrackSegment>,
) -> (Entity, [Entity; OUTLET_N]) {
    let mut end: Option<f32> = None;
    let mut groups: (Vec<Entity>, Vec<Entity>) = (Vec::default(), Vec::default());

    for e_neighbor in origin.neighbors.iter() {
        let segment = segments.get(e_neighbor).unwrap();
        let out_angle = segment.angle_from(e_origin).unwrap().clamp(-PI, PI);
        let out_angle = ((out_angle + PI) % (2.0 * PI)) - PI;

        match end {
            None => {
                end = Some(out_angle);
                groups.0.add(e_neighbor);
            }
            Some(end_angle) => {
                let diff = out_angle - end_angle;
                if diff > PI / -2.0 && diff < PI / 2.0 {
                    groups.0.add(e_neighbor);
                } else {
                    groups.1.add(e_neighbor);
                }
            }
        }
    }
    if groups.0.len() == 1 {
        (*groups.0.first().unwrap(), *groups.1.as_array().unwrap())
    } else {
        (*groups.1.first().unwrap(), *groups.0.as_array().unwrap())
    }
}

fn switch_clicked(clicked: On<TargetClicked>, mut switches: Query<&mut TrackSwitch>) {
    let e_switch = clicked.event().0;
    if let Ok(mut switch) = switches.get_mut(e_switch) {
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
            _ => {}
        }
    }
}
