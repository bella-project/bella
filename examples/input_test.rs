use bella::prelude::*;
use kurbo::Vec2;

fn hello(mut commands: Commands, mut instance: ResMut<BellaInstance>) {
    let scene = new_bella_scene(instance);

    for x in 0..100 {
        for y in 0..50 {
            commands.spawn(scene.stroke(
                kurbo::Stroke::new(6.0),
                peniko::Color::RED,
                BellaShape::RoundedRect(0.0, 0.0, 50.0, 50.0, 20.0),
                BellaTransform::from_xy(x as f64 * 50.0, y as f64 * 50.0),
            ));
        }
    }
}

pub fn update(
    time: Res<BellaTime>,
    input: Res<BellaInput>,
    mut transform_query: Query<&mut BellaTransform>,
) {
    for mut t in &mut transform_query {
        if input.is_key_pressed(KeyCode::KeyW) {
            t.add_translation(Vec2::new(0.0, -100.0 * time.delta_seconds()));
        }

        if input.is_key_pressed(KeyCode::KeyS) {
            t.add_translation(Vec2::new(0.0, 100.0 * time.delta_seconds()));
        }

        if input.is_key_pressed(KeyCode::KeyA) {
            t.add_translation(Vec2::new(-100.0 * time.delta_seconds(), 0.0));
        }

        if input.is_key_pressed(KeyCode::KeyD) {
            t.add_translation(Vec2::new(100.0 * time.delta_seconds(), 0.0));
        }
    }
}

pub fn main() {
    BellaApp::new("Bella: Input Test", 1280, 720)
        .on_start(hello)
        .on_update(update)
        .run();
}
