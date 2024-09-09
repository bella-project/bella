//! This is where the rendering of scenes and shapes happen by being a translation layer between Bella and [`vello`].

use bevy_ecs::prelude::*;

use vello::Scene;

use vello::peniko::{
    kurbo::{BezPath, Circle, Ellipse, Line, Point, RoundedRect, Shape, Stroke, Vec2},
    Brush, Fill,
};

use crate::prelude::BellaInstance;

use crate::transforms::BellaTransform;

/// Your scene in form of a reference. Used to communicate to [`BellaInstance`]'s `scenes`'s HashMap.
#[derive(Component)]
pub struct BellaScene {
    // Contains the ID, which was probably assigned by [`new_bella_scene`].
    id: usize,
}

/// The draw command you want to execute in your [`BellaScene`], in form of a component.
/// This is used as a translation layer between Bella and [`vello`]'s draw commands.
#[derive(Component)]
pub struct BellaCommand {
    /// Contains the ID, probably assigned by [`BellaScene`]'s draw functions.
    pub scene_id: usize,
    /// Contains the [`Stroke`] of your draw command. If it's [`None`], it'll not render a stroke.
    pub stroke: Option<Stroke>,
    /// Contains the [`Fill`] of your draw command. If it's [`None`], it'll not render a fill.
    pub fill: Option<Fill>,
    /// Contains the [`Brush`] of your draw command. It can be from a solid color, to any kind of gradient.
    pub brush: Brush,
    /// Contains the [`BellaShape`] of your draw command. It's used to dictate if its a circle, square, or any kind of shape you want.
    pub shape: BellaShape,
}

/// The draw command you want to execute in your [`BellaScene`], in form of a bundle.
/// Compared to [`BellaCommand`], this will add a [`BellaTransform`] to your entity. Allowing you to move, rotate, scale and transform your shape at runtime.
/// If you want more customization without the use of [`BellaScene`]'s functions, it's recommended to spawn this bundle instead of a single [`BellaCommand`].
#[derive(Bundle)]
pub struct BellaCommandBundle {
    pub command: BellaCommand,
    pub transform: BellaTransform,
}

/// The shape you want to be displayed. This is a translation layer between Bella and [`vello::kurbo`]'s shapes.
pub enum BellaShape {
    /// A Rounded Rectangle. The values you set (in order from left to right) are: `x`, `y`, `width`, `height` and `radius`. Can be translated to [`vello::kurbo::RoundedRect`].
    RoundedRect(f64, f64, f64, f64, f64),
    /// A Circle. The values you set (in order from left to right) are: `center` and `radius`. Can be translated to [`vello::kurbo::Circle`].
    Circle(Point, f64),
    /// An Ellipse. The values you set (in order from left to right) are: `center`, `radii` and `radius`. Can be translated to [`vello::kurbo::Ellipse`].
    Ellipse(Point, Vec2, f64),
    /// A Line. The values you set (in order from left to right) are: the `begin` point and the `end` point of the line. Can be translated to [`vello::kurbo::Line`].
    Line(Point, Point),
}

impl BellaShape {
    /// Translates your shape into a [`vello::kurbo`] [`Shape`]. It returns a [`BezPath`] to ensure to the compiler that shapes internally contain the same size.
    pub fn to_kurbo(&self) -> BezPath {
        match self {
            BellaShape::RoundedRect(x, y, width, height, radius) => {
                RoundedRect::new(*x, *y, *width, *height, *radius).to_path(0.01)
            }
            BellaShape::Circle(center, radius) => Circle::new(*center, *radius).to_path(0.01),
            BellaShape::Ellipse(center, radii, x_rotation) => {
                Ellipse::new(*center, *radii, *x_rotation).to_path(0.01)
            }
            BellaShape::Line(p0, p1) => Line::new(*p0, *p1).to_path(0.01),
        }
    }
}

/// Creates a new [`BellaScene`] and registers it in [`BellaInstance`].
pub fn new_bella_scene(mut root: ResMut<BellaInstance>) -> BellaScene {
    root.max_scene_id += 1;

    let ms = root.max_scene_id;

    root.scenes.insert(ms, Scene::new());

    BellaScene {
        id: root.max_scene_id,
    }
}

impl BellaScene {
    /// Creates a new stroke inside of the scene.
    ///
    /// - `stroke` sets the [`Stroke`] & its properties.
    /// - `brush` sets the [`Brush`] of the stroke. Can be a solid color, a linear gradient, etc.
    /// - `shape` sets the [`Shape`] of the stroke. Can be a cirle, square, bezier path, etc.
    /// - `transform` sets the stroke's [`BellaTransform`]. Used to move, rotate, scale and overall transform the shape.
    ///
    /// This is a translation layer between Bella and [`vello`]'s stroke command.
    pub fn stroke(
        &self,
        stroke: Stroke,
        brush: impl Into<Brush>,
        shape: BellaShape,
        transform: BellaTransform,
    ) -> BellaCommandBundle {
        BellaCommandBundle {
            command: BellaCommand {
                scene_id: self.id,
                stroke: Some(stroke),
                fill: None,
                brush: brush.into(),
                shape,
            },
            transform,
        }
    }

    /// Creates a new fill inside of the scene.
    ///
    /// - `fill` sets the [`Fill`] & its properties.
    /// - `brush` sets the [`Brush`] of the fill. Can be a solid color, a linear gradient, etc.
    /// - `shape` sets the [`Shape`] of the fill. Can be a cirle, square, bezier path, etc.
    /// - `transform` sets the fill's [`BellaTransform`]. Used to move, rotate, scale and overall transform the shape.
    ///
    /// This is a translation layer between Bella and [`vello`]'s fill command.
    pub fn fill(
        &self,
        fill: Fill,
        brush: impl Into<Brush>,
        shape: BellaShape,
        transform: BellaTransform,
    ) -> BellaCommandBundle {
        BellaCommandBundle {
            command: BellaCommand {
                scene_id: self.id,
                stroke: None,
                fill: Some(fill),
                brush: brush.into(),
                shape,
            },
            transform,
        }
    }
}

/// The main logic that renders shapes and scenes to the screen.
/// This is the bridge between Bella and [`vello`]. It gets all of the available [`BellaCommand`]s with their [`BellaTransform`]s
/// and converts all of the information those components provide into [`vello`] draw commands.
pub fn render(
    mut bella_query: Query<(&BellaCommand, &BellaTransform)>,
    mut instance: ResMut<BellaInstance>,
) {
    for (b, t) in &mut bella_query {
        let scene: &mut Scene = instance.scenes.get_mut(&b.scene_id).unwrap();

        if let Some(stroke) = &b.stroke {
            scene.stroke(stroke, t.affine, &b.brush, None, &b.shape.to_kurbo());
        }

        if let Some(fill) = &b.fill {
            scene.fill(*fill, t.affine, &b.brush, None, &b.shape.to_kurbo());
        }
    }
}
