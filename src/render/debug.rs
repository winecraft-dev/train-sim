use bevy::{color::palettes::css, prelude::*};

use crate::{
    landmark::Landmark,
    loc::{Direction, Location},
    signal::{
        ObservableBound, Signal,
        block::{Block, BlockBound, OccupiedBlock},
    },
    track::{TrackNode, TrackSegment, TrackVariant, switch::TrackSwitch},
    train::{
        Train,
        axle::{AXLE_DISTANCE, Axle},
    },
};

pub struct DebugRenderPlugin;

impl Plugin for DebugRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_config).add_systems(
            Update,
            (
                render_tracks,
                render_trains,
                render_signals,
                render_facing.run_if(should_render_directions),
                render_locations.run_if(should_render_locations),
                render_blocks.run_if(should_render_blocks),
                render_switches,
            )
                .chain(),
        );
    }
}

#[derive(Resource, Default)]
pub struct RenderConfig {
    pub directions: bool,
    pub locations: bool,
    pub blocks: bool,
}

fn should_render_directions(config: Res<RenderConfig>) -> bool {
    config.directions
}

fn should_render_locations(config: Res<RenderConfig>) -> bool {
    config.locations
}

fn should_render_blocks(config: Res<RenderConfig>) -> bool {
    config.blocks
}

fn init_config(mut commands: Commands) {
    commands.insert_resource(RenderConfig::default());
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
                gizmos.line_2d(a.xy(), b.xy(), css::DIM_GRAY);
            }
            TrackVariant::Curved {
                center,
                angle: _,
                radius: _,
            } => {
                let center = nodes.get(center).unwrap().translation;
                gizmos.short_arc_2d_between(center.xy(), a.xy(), b.xy(), css::DIM_GRAY);
            }
        };
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
                gizmos.rounded_rect_2d(position, Vec2::new(5.0, 5.0), css::DARK_CYAN);
                gizmos.arrow_2d(position, position + direction, css::BLUE);
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
                gizmos.rounded_rect_2d(position, Vec2::new(5.0, 5.0), css::DARK_CYAN);
                gizmos.arrow_2d(position, position + direction, css::BLUE);
            }
            _ => {}
        };
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
        let arrow_pos = (switch_pos - facing_pos).normalize() * 20.0 + facing_pos;
        gizmos.arrow_2d(facing_pos, arrow_pos, css::ORANGE);
    }
}

fn render_locations(
    mut gizmos: Gizmos,
    locations: Query<(&Transform, Option<&Landmark>), With<Location>>,
) {
    for (transform, landmark) in locations {
        let color = match landmark {
            Some(_) => css::PURPLE,
            None => css::HOT_PINK,
        };
        gizmos.circle_2d(transform.translation.xy(), 1.0, color);
    }
}

fn render_blocks(
    mut gizmos: Gizmos,
    blocks: Query<(Entity, &Block, Option<&OccupiedBlock>)>,
    children: Query<&Children>,
    bounds: Query<(&BlockBound, &Transform)>,
) {
    for (e_block, _, occupied) in blocks {
        let bound_pos: Vec<Vec2> = children
            .get(e_block)
            .unwrap()
            .iter()
            .filter_map(|e| bounds.get(e).ok())
            .map(|b| b.1.translation.xy())
            .collect();

        let color = match occupied {
            Some(_) => css::ORANGE_RED,
            None => css::YELLOW,
        };
        gizmos.circle_2d(bound_pos[0], 1.0, color);
        gizmos.circle_2d(bound_pos[1], 1.0, color);
    }
}

fn render_signals(
    mut gizmos: Gizmos,
    signals: Query<(&Transform, &Signal)>,
    obv_bounds: Query<&Transform, With<ObservableBound>>,
    blocks: Query<Option<&OccupiedBlock>>,
) {
    for (pos, signal) in signals {
        let pos = pos.translation.xy();
        let occupied = blocks.get(signal.block).unwrap();
        let color = match occupied {
            Some(_) => css::RED,
            None => css::GREEN,
        };

        gizmos.circle_2d(pos, 10.0, color);
    }

    for pos in obv_bounds {
        let pos = pos.translation.xy();
        gizmos.cross_2d(pos, 5.0, css::RED);
    }
}
