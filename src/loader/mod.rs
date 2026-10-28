use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;

use crate::{
    landmark::store::LandmarkStore,
    loader::wire::{Railway, TrackVariant},
    track::{
        SwitchesSpawned,
        builder::{TrackBuilder, TrackNodesBuilt},
        store::{NodeStore, SegmentStore},
    },
    train::{builder::TrainBuilder, store::AxleStore},
    zone::{
        builder::{ZoneBuilder, ZonesBuilt},
        store::{SwitchJunctionLookup, ZoneStore},
    },
};

pub mod wire;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YamlAssetPlugin::<Railway>::new(&["maps/demo1.yaml"]))
            .add_systems(PreStartup, load_railway)
            .add_systems(Update, railway_loaded)
            .add_observer(reset_railroad)
            .add_observer(load_track_nodes)
            .add_observer(load_track_segments)
            .add_observer(load_zones)
            .add_observer(load_trains);
    }
}

#[derive(Resource)]
struct RailwayHandle(Handle<Railway>);

fn load_railway(mut commands: Commands, asset_server: Res<AssetServer>) {
    let railway = RailwayHandle(asset_server.load::<Railway>("maps/demo1.yaml"));

    commands.insert_resource(railway);
}

fn railway_loaded(mut commands: Commands, mut events: MessageReader<AssetEvent<Railway>>) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id: _ } = event else {
            continue;
        };
        commands.trigger(RailwayReloaded);
    }
}

#[derive(Event)]
pub struct RailwayReloaded;

fn reset_railroad(
    _reloaded: On<RailwayReloaded>,
    mut commands: Commands,
    node_store: ResMut<NodeStore>,
    segment_store: ResMut<SegmentStore>,
    landmark_store: ResMut<LandmarkStore>,
    zone_store: ResMut<ZoneStore>,
    junction_lookup: ResMut<SwitchJunctionLookup>,
    axle_store: ResMut<AxleStore>,
) {
    NodeStore::reset(&mut commands, node_store);
    SegmentStore::reset(&mut commands, segment_store);
    LandmarkStore::reset(landmark_store);
    ZoneStore::reset(&mut commands, zone_store);
    SwitchJunctionLookup::reset(junction_lookup);
    AxleStore::reset(&mut commands, axle_store);

    commands.trigger(RailwayLoaded);
}

#[derive(Event)]
pub struct RailwayLoaded;

fn load_track_nodes(
    _loaded: On<RailwayLoaded>,
    mut builder: TrackBuilder,
    handle: Res<RailwayHandle>,
    railways: Res<Assets<Railway>>,
) {
    let Some(railway) = railways.get(&handle.0) else {
        return;
    };

    for node in railway.tracks.nodes.iter() {
        builder.node(node.x, node.y);
    }

    builder.nodes_built();
}

fn load_track_segments(
    _done: On<TrackNodesBuilt>,
    mut builder: TrackBuilder,
    handle: Res<RailwayHandle>,
    railways: Res<Assets<Railway>>,
) {
    let Some(railway) = railways.get(&handle.0) else {
        return;
    };

    for segment in railway.tracks.segments.iter() {
        let a = segment.nodes.0;
        let b = segment.nodes.1;
        match match segment.variant {
            TrackVariant::Straight => builder.straight(a, b),
            TrackVariant::Curved { center } => builder.curved(a, b, center),
        } {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to spawn segment {:?}: {:?}", segment, e);
                continue;
            }
        };
    }

    builder.built();
}

fn load_zones(
    _done: On<SwitchesSpawned>,
    mut builder: ZoneBuilder,
    handle: Res<RailwayHandle>,
    railways: Res<Assets<Railway>>,
) {
    let Some(railway) = railways.get(&handle.0) else {
        return;
    };

    for zone in railway.zones.iter() {
        match builder.from_wire(zone) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to spawn zone {:?}: {:?}", zone, e);
                continue;
            }
        };
    }

    builder.built();
}

fn load_trains(
    _done: On<ZonesBuilt>,
    mut builder: TrainBuilder,
    handle: Res<RailwayHandle>,
    railways: Res<Assets<Railway>>,
) {
    let Some(railway) = railways.get(&handle.0) else {
        return;
    };

    for train in railway.trains.iter() {
        match builder.from_wire(train) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to spawn train {:?}: {:?}", train, e);
                continue;
            }
        };
    }
}
