use bevy::{prelude::*, render::camera::ScalingMode};
use bevy::prelude::Srgba;

use bella::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 15.0,
        min_height: 15.0,
    };

    commands.spawn(camera);

    let bella_instance = BellaInstance::new(&mut commands);

    commands.spawn(
        bella_instance.shape(
            bella_svg("assets/joey/joey.svg", BellaExportOptions::ScaleToUnitKeepAspect(1.0)),
            Transform::default(),
        )
    ).with_children(|parent| {
        parent.spawn(
            bella_instance.shape(
                bella_text("Hello! I'm Joey! :D".to_string(), "assets/joey/FiraSans-Regular.ttf")
                    .with_stroke(BellaStroke::new(0.05))
                    .with_stroke_brush(BellaBrush::Solid(Color::Srgba(Srgba::rgba_u8(0, 0, 0, 0)))),
                Transform::from_xyz(2.0, 0.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 1.0)),
            )
        );
    });
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