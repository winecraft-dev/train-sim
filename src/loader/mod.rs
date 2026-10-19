use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;

use crate::{
    loader::wire::{Railway, TrackVariant},
    track::{
        SwitchesSpawned,
        builder::{TrackBuilder, TrackNodesBuilt},
    },
    zone::builder::ZoneBuilder,
};

pub mod wire;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YamlAssetPlugin::<Railway>::new(&["maps/demo1.yaml"]))
            .add_systems(PreStartup, load_railway)
            .add_systems(Update, railway_loaded)
            .add_observer(spawn_track_nodes)
            .add_observer(spawn_track_segments)
            .add_observer(spawn_zones);
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
        commands.trigger(RailwayLoaded);
    }
}

#[derive(Event)]
pub struct RailwayLoaded;

fn spawn_track_nodes(
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

fn spawn_track_segments(
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

fn spawn_zones(
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
