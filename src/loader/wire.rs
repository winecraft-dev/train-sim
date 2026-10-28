use bevy::{asset::Asset, prelude::*, reflect::TypePath};
use serde::Deserialize;

use crate::loc::Dir;

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct Railway {
    pub tracks: TrackData,
    pub zones: Vec<Zone>,
    pub trains: Vec<Train>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Zone {
    pub signal_distance: Option<f32>,
    pub observable_distance: Option<f32>,
    pub entrys: Vec<FLoc>,
    pub exits: Vec<FLoc>,
    pub switches: Option<Vec<usize>>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct FLoc {
    #[serde(flatten)]
    pub loc: Loc,
    pub dir: Dir,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Loc {
    pub track: usize,
    pub offset: Option<f32>,
    pub progress: Option<f32>,
    pub end: Option<Dir>,
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
    #[serde(flatten)]
    pub variant: TrackVariant,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TrackVariant {
    Straight,
    Curved { center: usize },
}

#[derive(Deserialize, Debug)]
pub struct Train {
    pub floc: FLoc,
    pub speed: Option<f32>,
}
