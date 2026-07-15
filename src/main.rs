use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{GREEN, RED, SEA_GREEN},
    math::VectorSpace,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, gen_track_mesh).chain())
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
pub struct Track {
    a: Vec2,
    b: Vec2,
}

impl Track {
    fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }
}

#[derive(Default)]
pub struct TrackBuilder {
    positions: Vec<Vec2>,
    indices: Vec<u32>,
}

impl TrackBuilder {
    fn add_track(&mut self, track: &Track) {
        self.positions.push(track.a);
        let a = self.positions.len();
        self.positions.push(track.b);
        let b = self.positions.len();

        self.indices.push(a as u32);
        self.indices.push(b as u32);
    }
}

const TRACK_WIDTH: f32 = 5.0;
impl MeshBuilder for TrackBuilder {
    fn build(&self) -> Mesh {
        let positions: Vec<Vec3> = self.positions.iter().map(|p| p.extend(0.0)).collect();
        let indices = Indices::U32(self.indices.clone());

        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_indices(indices)
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Camera::default()));

    let tracks = [
        Track::new(Vec2::new(0.0, 10.0), Vec2::new(-10.0, -10.0)),
        Track::new(Vec2::new(30.0, 0.0), Vec2::new(-30.0, 30.0)),
    ];

    for track in tracks {
        commands.spawn(track);
    }
}

fn gen_track_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tracks: Query<&Track>,
) {
    let center = meshes.add(Circle::new(3.0));
    commands.spawn((
        Mesh2d(center),
        MeshMaterial2d(materials.add(Color::Srgba(GREEN))),
    ));

    let mut track_builder = TrackBuilder::default();
    for track in tracks {
        track_builder.add_track(track);
    }

    let track_mesh = track_builder.build();
    let track_id = meshes.add(track_mesh);

    commands.spawn((
        Mesh2d(track_id),
        Transform::from_translation(Vec3::ZERO),
        MeshMaterial2d(materials.add(Color::Srgba(RED))),
    ));
}

fn rotate(mut tracks: Query<&mut Transform, With<Track>>) {
    for mut transform in &mut tracks {
        transform.rotate_z(0.01);
    }
}
