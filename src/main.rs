use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{GREEN, RED},
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

#[derive(Debug, Component)]
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

const TRACK_WIDTH: f32 = 5.0;
impl TrackBuilder {
    fn add_track(&mut self, track: &Track) {
        let delta = track.a - track.b;
        let scaled = delta.normalize() * (TRACK_WIDTH / 2.0);
        let rotated = Vec2::new(scaled.y, -scaled.x);
        println!("{:?} {} {}", track, &scaled, &rotated);

        let l_track_pos_a = track.a + rotated;
        let l_track_pos_b = track.b + rotated;
        let r_track_pos_a = track.a - rotated;
        let r_track_pos_b = track.b - rotated;

        self.positions.push(l_track_pos_a);
        let la = self.positions.len() - 1;
        self.positions.push(l_track_pos_b);
        let lb = self.positions.len() - 1;
        self.positions.push(r_track_pos_a);
        let ra = self.positions.len() - 1;
        self.positions.push(r_track_pos_b);
        let rb = self.positions.len() - 1;

        self.indices.push(la as u32);
        self.indices.push(lb as u32);
        self.indices.push(ra as u32);
        self.indices.push(rb as u32);
    }
}

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
        Track::new(Vec2::new(0.0, 0.0), Vec2::new(30.0, 30.0)),
        Track::new(Vec2::new(-300.0, 0.0), Vec2::new(0.0, 0.0)),
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
    let center = meshes.add(Circle::new(2.0));
    commands.spawn((
        Mesh2d(center),
        Transform::from_translation(Vec3::new(-1.0, 1.0, 0.0)),
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
