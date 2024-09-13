use bella::prelude::*;
use kurbo::Stroke;
use peniko::{Color, Fill};

fn spawn_simple(mut commands: Commands, instance: ResMut<BellaInstance>) {
    let scene = new_bella_scene(instance);

    // Draw an outlined rectangle
    let stroke = Stroke::new(6.0);
    let rect = BellaShape::RoundedRect(10.0, 10.0, 240.0, 240.0, 20.0);
    let rect_stroke_color = Color::rgb(0.9804, 0.702, 0.5294);
    commands.spawn(scene.stroke(
        stroke.clone(),
        rect_stroke_color,
        rect,
        BellaTransform::IDENTITY,
    ));

    // Draw a filled circle
    let circle = BellaShape::Circle((420.0, 200.0).into(), 120.0);
    let circle_fill_color = Color::rgb(0.9529, 0.5451, 0.6588);
    commands.spawn(scene.fill(
        Fill::NonZero,
        circle_fill_color,
        circle,
        BellaTransform::IDENTITY,
    ));

    // Draw a filled ellipse
    let ellipse = BellaShape::Ellipse((250.0, 420.0).into(), (100.0, 160.0).into(), -90.0);
    let ellipse_fill_color = Color::rgb(0.7961, 0.651, 0.9686);
    commands.spawn(scene.fill(
        Fill::NonZero,
        ellipse_fill_color,
        ellipse,
        BellaTransform::IDENTITY,
    ));

    // Draw a straight line
    let line = BellaShape::Line((260.0, 20.0).into(), (620.0, 100.0).into());
    let line_stroke_color = Color::rgb(0.5373, 0.7059, 0.9804);
    commands.spawn(scene.stroke(stroke, line_stroke_color, line, BellaTransform::IDENTITY));
}

pub fn main() {
    BellaApp::new("Bella: Vello Shapes", 800, 600)
        .new_bella_world()
        .on_start(spawn_simple)
        .run();
}
