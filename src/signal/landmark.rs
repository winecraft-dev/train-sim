use bevy::prelude::*;

use crate::{
    loc::{Direction, Location, cursor::TrackCursor},
    train::axle::AxleMoved,
};

pub struct LandmarkPlugin;

impl Plugin for LandmarkPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(check_landmarks_passed);
    }
}

#[derive(Component)]
pub struct Landmark;

#[derive(Event)]
pub struct LandmarkPassed {
    pub forwards: bool,
    pub landmark: Entity,
    pub train: Entity,
}

fn check_landmarks_passed(
    moved: On<AxleMoved>,
    mut commands: Commands,
    landmarks: Query<(Entity, &Location, &Direction), With<Landmark>>,
    cursor: TrackCursor,
) {
    let AxleMoved {
        train: e_train,
        from,
        to,
    } = *moved;

    for (e_landmark, loc, dir) in landmarks {
        let pass_dir = match cursor.passed(from, to, *loc) {
            Ok(p) => match p {
                Some(d) => d,
                None => continue,
            },
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        commands.trigger(LandmarkPassed {
            forwards: pass_dir == *dir,
            landmark: e_landmark,
            train: e_train,
        });
    }
}
