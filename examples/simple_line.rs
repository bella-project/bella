use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::prelude::Srgba;

use bella::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    let bella_instance = BellaInstance::new(&mut commands);

    commands.spawn(bella_instance.shape(
        bella_line()
            .with_stroke(BellaStroke::new(10.0))
            .with_begin(Vec2::new(-100.0, -100.0))
            .with_end(Vec2::new(100.0, 100.0)),
        Transform::default()
    ));
}

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins.set(ImagePlugin::default_nearest()),
                BellaPlugin,
            ))
        .add_systems(Startup, setup)
        .run();
}