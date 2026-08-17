use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_browsing)
            .add_systems(Update, handle_drag)
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
    mut camera: Single<&mut Transform, With<Camera>>,
) {
    if let DragState::Standby = drag_state.into_inner() {
        return;
    }

    for motion in mouse_reader.read() {
        let delta = motion.delta;
        let flipped = Vec2::new(-delta.x, delta.y);
        camera.translation += flipped.extend(0.0);
    }
}
