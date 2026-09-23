use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_browsing)
            .add_systems(Update, (handle_drag, zoom))
            .add_observer(drag_started)
            .add_observer(drag_ended);
    }
}

#[derive(Event)]
pub struct DragStarted;
#[derive(Event)]
pub struct DragEnded;

#[derive(Resource)]
pub enum DragState {
    Standby,
    Dragging,
}

fn init_browsing(mut commands: Commands) {
    commands.insert_resource(DragState::Standby);
}

fn drag_started(_started: On<DragStarted>, mut drag_state: ResMut<DragState>) {
    *drag_state = DragState::Dragging;
}

fn drag_ended(_ended: On<DragEnded>, mut drag_state: ResMut<DragState>) {
    *drag_state = DragState::Standby;
}

fn handle_drag(
    drag_state: Res<DragState>,
    mut mouse_reader: MessageReader<MouseMotion>,
    mut camera: Single<(&mut Transform, &Projection), With<Camera>>,
) {
    if let DragState::Standby = drag_state.into_inner() {
        return;
    }

    for motion in mouse_reader.read() {
        let ortho = match camera.1 {
            Projection::Orthographic(o) => o,
            _ => return,
        };
        let delta = motion.delta * ortho.scale;
        let flipped = Vec2::new(-delta.x, delta.y);
        camera.0.translation += flipped.extend(0.0);
    }
}

const ZOOM_SCALE: f32 = 0.001;

fn zoom(mut scroll: MessageReader<MouseWheel>, mut camera: Query<&mut Projection, With<Camera2d>>) {
    let Ok(projection) = camera.single_mut() else {
        return;
    };

    let ortho = match projection.into_inner() {
        Projection::Orthographic(o) => o,
        _ => return,
    };

    for e in scroll.read() {
        ortho.scale -= e.y * ZOOM_SCALE;
    }
    ortho.scale = ortho.scale.clamp(0.1, 5.0);
}
