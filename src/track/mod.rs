use bevy::{color::palettes::css::RED, prelude::*};
use track_mesh::TrackMeshBuilder;

mod track_mesh;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(calculate_track_data)
            .add_observer(generate_track_mesh);
    }
}

#[derive(Event)]
pub struct TrackUpdate;

#[derive(Debug, Component)]
pub struct TrackNode {
    pub position: Vec2,
}

impl TrackNode {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
        }
    }
}

#[derive(Debug)]
pub enum TrackVariant {
    Straight,
    Curved {
        center: Entity,
        angle: Option<f32>,
        radius: Option<f32>,
    },
}

#[derive(Debug, Component)]
pub struct TrackSegment {
    pub nodes: (Entity, Entity),
    pub variant: TrackVariant,

    pub length: Option<f32>,
}

impl TrackSegment {
    pub fn straight(nodes: (Entity, Entity)) -> Self {
        Self {
            nodes,
            variant: TrackVariant::Straight,

            length: None,
        }
    }

    pub fn curved(nodes: (Entity, Entity), center: Entity) -> Self {
        Self {
            nodes,
            variant: TrackVariant::Curved {
                center,
                angle: Option::None,
                radius: Option::None,
            },

            length: None,
        }
    }

    fn calculate_length(&mut self, nodes: &Query<&TrackNode>) {
        let a = nodes.get(self.nodes.0).unwrap().position;
        let b = nodes.get(self.nodes.1).unwrap().position;

        let length = match &mut self.variant {
            TrackVariant::Straight => (a - b).length(),
            TrackVariant::Curved {
                center,
                angle,
                radius,
            } => {
                let center = nodes.get(*center).unwrap().position;
                let angle_a = (a - center).to_angle();
                let angle_b = (b - center).to_angle();
                let calc_angle = angle_b - angle_a;
                let calc_radius = (a - center).length();

                *angle = Some(calc_angle);
                *radius = Some(calc_radius);

                calc_angle * calc_radius
            }
        };
        self.length = Some(length);
    }
}

pub fn calculate_track_data(
    _track_updated: On<TrackUpdate>,
    nodes: Query<&TrackNode>,
    segments: Query<&mut TrackSegment>,
) {
    for mut segment in segments {
        segment.calculate_length(&nodes);
    }
}

pub fn generate_track_mesh(
    _track_updated: On<TrackUpdate>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    nodes: Query<(Entity, &TrackNode)>,
    segments: Query<&TrackSegment>,
) {
    let mut track_builder = TrackMeshBuilder::default();
    for (entity, node) in nodes {
        track_builder.add_node(entity, node);
    }
    for segment in segments {
        match segment.variant {
            TrackVariant::Straight => track_builder.add_straight_track(segment.nodes),
            TrackVariant::Curved {
                center,
                angle,
                radius,
            } => track_builder.add_curved_track(
                segment.nodes,
                center,
                angle.unwrap(),
                radius.unwrap(),
            ),
        }
    }

    let track_mesh = track_builder.build();
    let track_id = meshes.add(track_mesh.clone());

    commands.spawn((
        Mesh2d(track_id),
        Transform::from_translation(Vec3::ZERO),
        MeshMaterial2d(materials.add(Color::Srgba(RED))),
    ));
}
