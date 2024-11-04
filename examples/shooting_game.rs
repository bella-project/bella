use bella::prelude::*;
use kurbo::{Affine, Vec2};
use peniko::{Color, Fill, Font};
use rand::Rng;

#[derive(Resource)]
struct GameManager {
    target: Vec2,
    target_radius: f64,
    score: usize,
    // timer: f64,
    game_font: Font,
}

fn start(mut commands: Commands, mut instance: ResMut<Instance>) {
    commands.insert_resource(GameManager {
        target: Vec2::new(300.0, 300.0),
        target_radius: 50.0,
        score: 0,
        // timer: 0.0,
        game_font: instance
            .asset_server()
            .load_file::<Font>("examples/assets/FiraSans-Regular.ttf")
            .unwrap()
            .clone(),
    });

    instance.new_scene("scene");
}

fn update(mut manager: ResMut<GameManager>, input: Res<Input>, instance: Res<Instance>) {
    if input.is_mouse_button_down(MouseButton::Left) {
        let mouse_to_target = distance_between(*input.mouse_position(), manager.target);

        if mouse_to_target < manager.target_radius {
            manager.score += 1;
            manager.target.x = rand::thread_rng()
                .gen_range(manager.target_radius..instance.resolution().x - manager.target_radius);
            manager.target.y = rand::thread_rng()
                .gen_range(manager.target_radius..instance.resolution().y - manager.target_radius);
        }
    }
}

fn draw(manager: Res<GameManager>, mut instance: ResMut<Instance>) {
    let scene = instance.get_scene("scene").unwrap();

    scene.fill_text(
        &manager.score.to_string(),
        Fill::NonZero,
        &manager.game_font,
        Affine::translate(Vec2::new(0.0, 0.0)),
        Color::WHITE,
        80.0,
    );

    scene.fill_circle(
        Fill::NonZero,
        Affine::translate(manager.target),
        Color::RED,
        manager.target_radius,
    );
}

fn distance_between(a: Vec2, b: Vec2) -> f64 {
    f64::sqrt((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0))
}

fn main() {
    App::new("Shooting Game", 1280, 720)
        .new_world()
        .on_start(start)
        .on_draw(draw)
        .on_update(update)
        .run();
}
