use bevy::{
    math::bounding::{BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

mod browse;

use browse::BrowsingPlugin;

use crate::control::browse::{DragEnded, DragStarted};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BrowsingPlugin)
            .add_systems(Update, handle_click);
    }
}

#[derive(Component)]
pub struct ClickTarget;

#[derive(Event)]
pub struct TargetClicked(pub Entity);

const TARGET_RADIUS: f32 = 5.0;
const CURSOR_RADIUS: f32 = 5.0;

fn handle_click(
    mut commands: Commands,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    click_input: Res<ButtonInput<MouseButton>>,
    targets: Query<(Entity, &Transform), With<ClickTarget>>,
) {
    if click_input.just_pressed(MouseButton::Left) {
        if let Some(target) = check_targets_clicked(window, camera, targets) {
            commands.trigger(TargetClicked(target));
            return;
        }
        commands.trigger(DragStarted);
    }
    if click_input.just_released(MouseButton::Left) {
        commands.trigger(DragEnded);
    }
}

fn check_targets_clicked(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    targets: Query<(Entity, &Transform), With<ClickTarget>>,
) -> Option<Entity> {
    let (camera, c_transform) = camera.into_inner();
    let mouse_position = match window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(c_transform, cursor).ok())
    {
        Some(mp) => mp,
        None => return None,
    };

    for (entity, transform) in targets {
        let target_circle = BoundingCircle::new(transform.translation.xy(), TARGET_RADIUS);
        let cursor_circle = BoundingCircle::new(mouse_position, CURSOR_RADIUS);

        if target_circle.contains(&cursor_circle) || target_circle.intersects(&cursor_circle) {
            return Some(entity);
        }
    }
    None
}
