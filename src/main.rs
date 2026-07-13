use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{GREEN, SEA_GREEN},
    math::VectorSpace,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
pub struct Track;

pub struct TrackBuilder {
    a: Vec2,
    b: Vec2,
}

impl TrackBuilder {
    fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    fn bundle(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> impl Bundle {
        let track = meshes.add(self.build());

        (
            Track,
            Transform::from_xyz(self.a.x, self.a.y, 0.0),
            Mesh2d(track),
            MeshMaterial2d(materials.add(Color::linear_rgba(255.0, 255.0, 255.0, 1.0))),
        )
    }
}

const TRACK_WIDTH: f32 = 5.0;
impl MeshBuilder for TrackBuilder {
    fn build(&self) -> Mesh {
        // normalize a and b to a
        let a = Vec3::ZERO;
        let b = (self.b - self.a).extend(0.0);
        let offset = Vec2::new(0.0, TRACK_WIDTH).extend(0.0);
        let positions = vec![a, b, a + offset, b + offset];
        let indices = Indices::U32(vec![0, 1, 2, 3]);

        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_indices(indices)
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, Camera::default()));

    let tracks = [
        TrackBuilder::new(Vec2::new(0.0, 0.0), Vec2::new(30.0, 30.0)),
        TrackBuilder::new(Vec2::new(-10.0, 10.0), Vec2::new(10.0, -10.0)),
        TrackBuilder::new(Vec2::new(-30.0, 0.0), Vec2::new(30.0, 30.0)),
    ];
    for track in tracks {
        commands.spawn(track.bundle(&mut meshes, &mut materials));
    }

    let center = meshes.add(Circle::new(3.0));
    commands.spawn((
        Mesh2d(center),
        MeshMaterial2d(materials.add(Color::Srgba(GREEN))),
    ));

    let test_line = Segment2d::new(Vec2::new(-30.0, 0.0), Vec2::new(30.0, 30.0));
    let mesh: Mesh = test_line.into();
    println!("{:?}", mesh);
}

fn rotate(mut tracks: Query<&mut Transform, With<Track>>) {
    for mut transform in &mut tracks {
        transform.rotate_z(0.01);
    }
}
