use bevy::{color::palettes::css, prelude::*};

use crate::{
    loc::{Direction, Location},
    signal::block::{Block, BlockBound, OccupiedBlock},
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
                render_bounds,
                render_facing,
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
    blocks: Query<(Entity, &Block, Option<&OccupiedBlock>)>,
    children: Query<&Children>,
    bounds: Query<(&BlockBound, &Transform)>,
) {
    for (e_block, _, occupied) in blocks {
        let children = children.get(e_block).unwrap();
        let e_bounds: [Entity; 2] = *children.as_array::<2>().unwrap();
        let bound_pos = bounds
            .get_many(e_bounds)
            .unwrap()
            .map(|(_, t)| t.translation.xy());

        let color = match occupied {
            Some(_) => css::ORANGE_RED,
            None => css::YELLOW,
        };
        gizmos.line_2d(bound_pos[0], bound_pos[1], color);
        gizmos.circle_2d(bound_pos[0], 5.0, color);
        gizmos.circle_2d(bound_pos[1], 5.0, color);
    }
}

fn render_facing(
    mut gizmos: Gizmos,
    facing: Query<(&Location, &Direction, &Transform)>,
    segments: Query<&TrackSegment>,
    switches: Query<&Transform>,
) {
    for (loc, facing, facing_pos) in facing {
        let facing_pos = facing_pos.translation.xy();
        let segment = segments.get(loc.track).unwrap();
        let facing_switch = match facing {
            Direction::FacingA => segment.nodes.0,
            Direction::FacingB => segment.nodes.1,
        };
        let switch_pos = switches.get(facing_switch).unwrap().translation.xy();
        let arrow_pos = (switch_pos - facing_pos).normalize() * 30.0 + facing_pos;
        gizmos.arrow_2d(facing_pos, arrow_pos, css::WHITE);
    }
}
