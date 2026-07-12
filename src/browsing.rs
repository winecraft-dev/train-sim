use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_browsing)
            .add_systems(Update, (start_drag, handle_drag));
    }
}

#[derive(Resource)]
pub enum DragState {
    Standby,
    Dragging,
}

fn init_browsing(mut commands: Commands) {
    commands.insert_resource(DragState::Standby);
}

fn start_drag(mut drag_state: ResMut<DragState>, click_input: Res<ButtonInput<MouseButton>>) {
    match *drag_state {
        DragState::Standby => {
            if click_input.just_pressed(MouseButton::Left) {
                *drag_state = DragState::Dragging;
            }
        }
        DragState::Dragging => {
            if click_input.just_released(MouseButton::Left) {
                *drag_state = DragState::Standby;
            }
        }
    }
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
