use bella::prelude::*;
use kurbo::{Affine, Circle, Ellipse, Line, RoundedRect, Stroke};
use peniko::{Color, Fill};

fn spawn_scene(mut instance: ResMut<Instance>) {
    instance.new_scene("scene1");
}

fn draw_simple(mut instance: ResMut<Instance>) {
    let scene = instance.get_scene("scene1").unwrap();

    // Draw an outlined rectangle
    let stroke = Stroke::new(6.0);
    let rect = RoundedRect::new(10.0, 10.0, 240.0, 240.0, 20.0);
    let rect_stroke_color = Color::rgb(0.9804, 0.702, 0.5294);
    scene.stroke(&stroke, Affine::IDENTITY, rect_stroke_color, None, &rect);

    // Draw a filled circle
    let circle = Circle::new((420.0, 200.0), 120.0);
    let circle_fill_color = Color::rgb(0.9529, 0.5451, 0.6588);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        circle_fill_color,
        None,
        &circle,
    );

    // Draw a filled ellipse
    let ellipse = Ellipse::new((250.0, 420.0), (100.0, 160.0), -90.0);
    let ellipse_fill_color = Color::rgb(0.7961, 0.651, 0.9686);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        ellipse_fill_color,
        None,
        &ellipse,
    );

    // Draw a straight line
    let line = Line::new((260.0, 20.0), (620.0, 100.0));
    let line_stroke_color = Color::rgb(0.5373, 0.7059, 0.9804);
    scene.stroke(&stroke, Affine::IDENTITY, line_stroke_color, None, &line);
}

pub fn main() {
    App::new("Bella: Vello Shapes", 800, 600)
        .new_world()
        .on_start(spawn_scene)
        .on_draw(draw_simple)
        .run();
}
