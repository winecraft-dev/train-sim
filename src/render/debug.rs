use bevy::{
    color::palettes::css::{self, WHITE},
    prelude::*,
};

use crate::{
    switch::TrackSwitch,
    track::{TrackNode, TrackSegment, TrackVariant},
    train::{Train, axle::Axle},
};

pub struct DebugRenderPlugin;

impl Plugin for DebugRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (render_tracks, render_switches, render_trains).chain(),
        );
    }
}

fn render_tracks(
    mut gizmos: Gizmos,
    nodes: Query<&Transform, With<TrackNode>>,
    segments: Query<&TrackSegment>,
) {
    for segment in segments {
        let a = nodes.get(segment.nodes.0).unwrap().translation;
        let b = nodes.get(segment.nodes.1).unwrap().translation;
        match segment.variant {
            TrackVariant::Straight => {
                gizmos.line_2d(a.xy(), b.xy(), css::RED);
            }
            TrackVariant::Curved {
                center,
                angle: _,
                radius: _,
            } => {
                let center = nodes.get(center).unwrap().translation;
                gizmos.short_arc_2d_between(center.xy(), a.xy(), b.xy(), css::RED);
            }
        };
    }
}

fn render_trains(
    mut gizmos: Gizmos,
    trains: Query<(Entity, &Transform), (With<Axle>, With<Train>)>,
    children: Query<&Children>,
    rear_axles: Query<&Transform, (With<Axle>, Without<Train>)>,
) {
    for (e_main, main_axle) in trains {
        let e_rear = children.get(e_main).unwrap()[0];
        let rear_axle = rear_axles.get(e_rear).unwrap();

        let main_pos = main_axle.translation.xy();
        let rear_pos = rear_axle.translation.xy();
        let arrow_pos = (main_pos - rear_pos).normalize() * 30.0 + main_pos;

        gizmos.circle_2d(main_pos, 10.0, css::BLUE);
        gizmos.circle_2d(rear_pos, 10.0, css::AQUAMARINE);
        gizmos.arrow_2d(main_pos, arrow_pos, css::WHITE);
    }
}

fn render_switches(
    mut gizmos: Gizmos,
    segments: Query<&TrackSegment>,
    switches: Query<(Entity, &Transform, &TrackSwitch)>,
    nodes: Query<&Transform, With<TrackNode>>,
) {
    for (e_switch, transform, switch) in switches {
        let position = transform.translation.xy();
        match switch {
            TrackSwitch::Switch {
                control,
                inlet: _,
                outlet,
            } => {
                // repeated block of code :3
                let active = outlet[*control];
                let active_segment = segments.get(active).unwrap();
                let select_node = active_segment.opposite(e_switch).unwrap();
                let select_pos = nodes.get(select_node).unwrap().translation.xy();
                let direction = (select_pos - position).normalize() * 55.0;
                gizmos.arrow_2d(position, position + direction, css::WHITE);
            }
            TrackSwitch::ThreewayTurnout {
                control,
                inlet: _,
                outlet,
            } => {
                let active = outlet[*control];
                let active_segment = segments.get(active).unwrap();
                let select_node = active_segment.opposite(e_switch).unwrap();
                let select_pos = nodes.get(select_node).unwrap().translation.xy();
                let direction = (select_pos - position).normalize() * 55.0;
                gizmos.arrow_2d(position, position + direction, css::WHITE);
            }
            _ => {}
        };
        gizmos.circle_2d(position, 8.0, css::GREEN);
    }
}
