#[derive(Component)]
pub struct RotationControl;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 15.0,
        min_height: 15.0,
    };

    commands.spawn(camera);

    let bella_instance = BellaInstance::new(&mut commands);

    commands.spawn(bella_instance.shape(
            bella_text("Press A or D to rotate the yellow object.".to_string(), "assets/joey/FiraSans-Regular.ttf")
                .with_stroke_brush(BellaBrush::Solid(Color::Srgba(Srgba::rgba_u8(0, 0, 0, 0)))),
            Transform::from_xyz(0.0, -4.0, 0.0).with_scale(Vec3::new(0.3, 0.3, 1.0)),
        )
    );

    commands.spawn((
        bella_instance.shape(
            bella_rect()
                .with_stroke(BellaStroke::new(0.1))
                .with_size(Vec2::new(1.0, 1.0))
                .with_radius_non_uniform(0.25, 0.25, 1.0, 1.0)
                .with_stroke_brush(BellaBrush::Solid(Color::Srgba(Srgba::rgba_u8(255, 255, 0, 255))))
                .with_fill_brush(BellaBrush::Solid(Color::Srgba(Srgba::rgba_u8(0, 0, 0, 0)))),
            Transform::default().with_scale(Vec3::new(4.0, 4.0, 1.0))
        ),
        RotationControl
    )).with_children(|parent| {
        for _i in 0..1000 {
            parent.spawn(
                bella_instance.shape(
                    bella_rect()
                        .with_stroke(BellaStroke::new(0.1))
                        .with_size(Vec2::new(1.0, 1.0))
                        .with_radius(1.0)
                        .with_fill_brush(BellaBrush::Solid(Color::Srgba(Srgba::rgba_u8(0, 0, 0, 0)))),
                    Transform::from_xyz(-2.0, 0.0, 0.0),
                )
            );
        }
    });
}

fn rotation_update(mut query: Query<&mut Transform, With<RotationControl>>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>) {

    for mut q in &mut query {
        if input.pressed(KeyCode::KeyA) {
            q.rotate_z(-3.0 * time.delta_seconds());
        }

        if input.pressed(KeyCode::KeyD) {
            q.rotate_z(3.0 * time.delta_seconds());
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins.set(ImagePlugin::default_nearest()),
                BellaPlugin,
            ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotation_update)
        .run();
}