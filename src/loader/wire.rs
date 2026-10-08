use bevy::{asset::Asset, prelude::*, reflect::TypePath};
use serde::Deserialize;

use crate::{
    loc::{self, Dir},
    track::builder::TrackStore,
};

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct Railway {
    pub tracks: TrackData,
    pub zones: Vec<Zone>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Zone {
    pub signal_distance: Option<f32>,
    pub observable_distance: Option<f32>,
    pub entry_locs: Vec<FLoc>,
    pub exit_locs: Vec<FLoc>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct FLoc {
    pub loc: Loc,
    pub dir: Dir,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Loc {
    pub track: usize,
    pub distance: f32,
}

impl FLoc {
    pub fn convert(self, store: Res<TrackStore>) -> Option<loc::FLoc> {
        let e_segment = store.segment(self.loc.track)?;
        Some((
            loc::Loc {
                track: e_segment,
                distance: self.loc.distance,
            },
            self.dir,
        ))
    }
}

#[derive(Deserialize, Debug)]
pub struct TrackData {
    pub nodes: Vec<TrackNode>,
    pub segments: Vec<TrackSegment>,
}

#[derive(Deserialize, Debug)]
pub struct TrackNode {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug)]
pub struct TrackSegment {
    pub nodes: (usize, usize),
    pub variant: TrackVariant,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TrackVariant {
    Straight,
    Curved { center: usize },
}
