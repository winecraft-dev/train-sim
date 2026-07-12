use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct Track;

pub struct TrackGenerator {
    a: Vec2,
    b: Vec2,
}

impl TrackGenerator {
    fn ends(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    fn generate(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> impl Bundle {
        let track = meshes.add(Segment2d::new(self.a, self.b));

        (
            Track,
            Transform::from_xyz(self.a.x, self.a.y, 0.0),
            Mesh2d(track),
            MeshMaterial2d(materials.add(Color::linear_rgba(255.0, 255.0, 255.0, 1.0))),
        )
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, Camera::default()));

    let tracks = [
        TrackGenerator::ends(Vec2::new(0.0, 0.0), Vec2::new(30.0, 30.0)),
        TrackGenerator::ends(Vec2::new(-30.0, 0.0), Vec2::new(30.0, 30.0)),
    ];
    for track in tracks {
        commands.spawn(track.generate(&mut meshes, &mut materials));
    }
}
