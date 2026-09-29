use std::f32::consts::PI;

use bevy::{ecs::relationship::RelationshipSourceCollection, prelude::*};

use crate::{
    control::{ClickTarget, TargetClicked},
    track::{NodeNeighborsComputed, SwitchesSpawned, TrackNode, TrackSegment, error::TrackError},
};

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_switches)
            .add_observer(switch_clicked);
    }
}

#[derive(Default, Debug, Component)]
pub enum NodeVariant {
    #[default]
    None,

    #[allow(dead_code)]
    Terminus(Entity), // use this TrackSegment later down the line...
    Track(Entity, Entity),
    Switch {
        control: usize,
        inlet: Entity,
        outlet: Vec<Entity>,
    },
}

impl NodeVariant {
    pub fn next_segment(&self, current: Entity) -> Option<Entity> {
        match self {
            NodeVariant::Track(a, b) => {
                if *a == current {
                    return Some(*b);
                } else if *b == current {
                    return Some(*a);
                }
            }
            NodeVariant::Switch {
                control,
                inlet,
                outlet,
            } => {
                if *inlet == current {
                    return Some(outlet[*control].clone());
                } else {
                    return Some(*inlet);
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
                continue;
            }
            1 => {
                let e_terminating_track = origin.neighbors[0];
                NodeVariant::Terminus(e_terminating_track)
            }
            2 => {
                let e_segment_a = origin.neighbors[0];
                let e_segment_b = origin.neighbors[1];
                NodeVariant::Track(e_segment_a, e_segment_b)
            }
            3 | 4 => {
                let (inlet, outlet) = match split_ports(e_origin, origin, segments) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("problem splitting ports: {:?}", e);
                        continue;
                    }
                };
                NodeVariant::Switch {
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

fn split_ports(
    e_origin: Entity,
    origin: &TrackNode,
    segments: Query<&TrackSegment>,
) -> Result<(Entity, Vec<Entity>), TrackError> {
    let mut end: Option<f32> = None;
    let mut groups: (Vec<Entity>, Vec<Entity>) = (Vec::default(), Vec::default());

    for e_neighbor in origin.neighbors.iter() {
        let Ok(segment) = segments.get(e_neighbor) else {
            return Err(TrackError::BrokenSegmentReference(e_neighbor));
        };
        let Some(out_angle) = segment.angle_from(e_origin) else {
            return Err(TrackError::NodeNotOfSegment(e_origin, e_neighbor));
        };
        let out_angle = ((out_angle + PI) % (2.0 * PI)) - PI;

        match end {
            None => {
                end = Some(out_angle);
                groups.0.push(e_neighbor);
            }
            Some(end_angle) => {
                let diff = out_angle - end_angle;
                if diff > PI / -2.0 && diff < PI / 2.0 {
                    groups.0.push(e_neighbor);
                } else {
                    groups.1.push(e_neighbor);
                }
            }
        }
    }

    if groups.0.len() == 1 {
        Ok((groups.0[0], groups.1))
    } else {
        Ok((groups.1[0], groups.0))
    }
}

fn switch_clicked(
    clicked: On<TargetClicked>,
    mut commands: Commands,
    mut switches: Query<&mut NodeVariant>,
) {
    let e_switch = clicked.event().0;
    if let Ok(mut switch) = switches.get_mut(e_switch) {
        match &mut *switch {
            NodeVariant::Switch {
                control,
                inlet: _,
                outlet,
            } => {
                *control = (*control + 1) % outlet.len();
                commands.trigger(SwitchUpdate {
                    switch: e_switch,
                    control: *control,
                });
            }
            _ => {}
        }
    }
}

// TEST, DELETE SOON
#[derive(Event)]
pub struct SwitchUpdate {
    pub switch: Entity,
    pub control: usize,
}
