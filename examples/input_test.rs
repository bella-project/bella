use bella::prelude::*;
use kurbo::Vec2;

#[derive(Component)]
struct MovingObject;

fn start(mut commands: Commands, mut instance: ResMut<Instance>) {
    instance.new_scene("scene1");

    for x in 0..5 {
        for y in 0..5 {
            commands.spawn((
                MovingObject,
                Transform::from_xy(x as f64 * 50.0, y as f64 * 50.0),
            ));
        }
    }
}

fn draw(mut moving_query: Query<(&MovingObject, &mut Transform)>, mut instance: ResMut<Instance>) {
    let scene = instance.get_scene("scene1").unwrap();

    for (m, mut t) in &mut moving_query {
        scene.stroke(
            &kurbo::Stroke::new(6.0),
            t.affine,
            peniko::Color::RED,
            None,
            &kurbo::RoundedRect::new(0.0, 0.0, 50.0, 50.0, 20.0),
        );
    }
}

pub fn update(time: Res<Time>, input: Res<Input>, mut transform_query: Query<&mut Transform>) {
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
    App::new("Bella: Input Test", 1280, 720)
        .new_world()
        .on_start(start)
        .on_draw(draw)
        .on_update(update)
        .run();
}
