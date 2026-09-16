use bevy::prelude::*;

pub mod cursor;
pub mod error;
pub mod locator;
pub mod projector;

use crate::loc::projector::Projector;

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (add_transforms).chain());
    }
}

pub type FacingLocation = (Loc, Dir);

#[derive(Component, Debug, Clone, Copy)]
pub struct Loc {
    pub track: Entity,
    pub distance: f32,
}

fn add_transforms(
    mut commands: Commands,
    locations: Query<(Entity, &Loc), Without<Transform>>,
    projector: Projector,
) {
    for (e, loc) in locations {
        let pos = match projector.project(*loc) {
            Ok(v3) => v3,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        commands.entity(e).insert(Transform::from_translation(pos));
    }
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    FacingA,
    #[default]
    FacingB,
}

impl Dir {
    pub fn flip(self) -> Self {
        match self {
            Dir::FacingA => Dir::FacingB,
            Dir::FacingB => Dir::FacingA,
        }
    }
}
