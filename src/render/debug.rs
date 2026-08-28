use bevy::{color::palettes::css, prelude::*};

use crate::{
    loc::{Direction, Location},
    signal::block::BlockBound,
    switch::TrackSwitch,
    track::{TrackNode, TrackSegment, TrackVariant},
    train::{
        Train,
        axle::{AXLE_DISTANCE, Axle},
    },
};

pub struct DebugRenderPlugin;

impl Plugin for DebugRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_tracks,
                render_switches,
                render_axles,
                render_trains,
                render_traversing,
                render_bounds,
            )
                .chain(),
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
                gizmos.line_2d(a.xy(), b.xy(), css::SLATE_BLUE);
            }
            TrackVariant::Curved {
                center,
                angle: _,
                radius: _,
            } => {
                let center = nodes.get(center).unwrap().translation;
                gizmos.short_arc_2d_between(center.xy(), a.xy(), b.xy(), css::SLATE_BLUE);
            }
        };
    }
}

fn render_axles(
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

        gizmos.circle_2d(main_pos, 10.0, css::DARK_RED);
        gizmos.circle_2d(rear_pos, 10.0, css::RED);
    }
}

fn render_traversing(
    mut gizmos: Gizmos,
    axles: Query<(&Axle, &Location, &Transform)>,
    segments: Query<&TrackSegment>,
    switches: Query<&Transform>,
) {
    for (_, loc, axle_pos) in axles {
        let axle_pos = axle_pos.translation.xy();
        let segment = segments.get(loc.track).unwrap();
        let facing_switch = match loc.direction {
            Direction::FacingA => segment.nodes.0,
            Direction::FacingB => segment.nodes.1,
        };
        let switch_pos = switches.get(facing_switch).unwrap().translation.xy();
        let arrow_pos = (switch_pos - axle_pos).normalize() * 30.0 + axle_pos;
        gizmos.arrow_2d(axle_pos, arrow_pos, css::WHITE);
    }
}

fn render_trains(
    mut gizmos: Gizmos,
    trains: Query<(Entity, &Transform), With<Train>>,
    children: Query<&Children>,
    rear_axles: Query<&Transform, (With<Axle>, Without<Train>)>,
) {
    for (e_train, train_pos) in trains {
        let main_pos = train_pos.translation.xy();

        let e_rear = children.get(e_train).unwrap()[0];
        let rear_pos = rear_axles.get(e_rear).unwrap().translation.xy();

        let center = main_pos - (main_pos - rear_pos) / 2.0;
        let angle = (main_pos - rear_pos).to_angle();
        let arrow_pos = (main_pos - rear_pos).normalize() * 30.0 + main_pos;
        gizmos.rect_2d(
            Isometry2d::new(
                center, // position
                Rot2::radians(angle),
            ),
            Vec2::new(AXLE_DISTANCE + 20.0, 20.0),
            css::GRAY,
        );
        gizmos.arrow_2d(center, arrow_pos, css::LIGHT_BLUE);
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

fn render_bounds(
    mut gizmos: Gizmos,
    bounds: Query<(&BlockBound, &Location, &Transform)>,
    segments: Query<&TrackSegment>,
    switches: Query<&Transform>,
) {
    for (_, loc, bound_pos) in bounds {
        let bound_pos = bound_pos.translation.xy();
        let segment = segments.get(loc.track).unwrap();
        let facing_switch = match loc.direction {
            Direction::FacingA => segment.nodes.0,
            Direction::FacingB => segment.nodes.1,
        };
        let switch_pos = switches.get(facing_switch).unwrap().translation.xy();
        let arrow_pos = (switch_pos - bound_pos).normalize() * 30.0 + bound_pos;
        gizmos.circle_2d(bound_pos, 5.0, css::YELLOW);
        gizmos.arrow_2d(bound_pos, arrow_pos, css::YELLOW);
    }
}
