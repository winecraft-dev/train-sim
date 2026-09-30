use serde::Deserialize;

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
