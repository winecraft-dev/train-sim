use bevy::{asset::Asset, reflect::TypePath};
use serde::Deserialize;

use crate::track::wire::TrackData;

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct Railway {
    pub tracks: TrackData,
}
