use bevy::{color::palettes::css::RED, prelude::*};

use crate::{
    render::track_mesh::TrackMeshBuilder,
    track::{TrackDataCalculated, TrackNode, TrackSegment, TrackVariant},
};

pub mod debug;
mod track_mesh;

#[allow(unused)]
pub struct MeshRenderPlugin;

impl Plugin for MeshRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(generate_track_mesh);
    }
}

#[allow(unused)]
pub fn generate_track_mesh(
    _track_updated: On<TrackDataCalculated>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    nodes: Query<(Entity, &Transform), With<TrackNode>>,
    segments: Query<&TrackSegment>,
) {
    let mut track_builder = TrackMeshBuilder::default();
    for (entity, transform) in nodes {
        track_builder.add_node(entity, transform.translation.xy());
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
