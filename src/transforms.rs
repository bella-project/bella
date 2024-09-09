//! Everything related to translation, rotation, scaling and any kind of transformation.

use bevy_ecs::prelude::*;

use vello::peniko::{
    kurbo::{Affine, Point, Vec2},
    BlendMode, Blob, Brush, BrushRef, Color, ColorStop, ColorStops, ColorStopsSource, Compose,
    Extend, Fill, Font, Gradient, Image, Mix, StyleRef,
};

/// Describes the position, rotation, scale and any kind of transformation of an entity. This is a translation layer between Bella and [`vello::kurbo`]'s [`Affine`].
#[derive(Component, Default)]
pub struct BellaTransform {
    pub affine: Affine,
}

impl BellaTransform {

	// Default transform.
    pub const IDENTITY: Self = Self {
        affine: Affine::IDENTITY,
    };

    // Creates a new Transform at position (x, y).
    pub fn from_xy(x: f64, y: f64) -> Self {
        Self {
            affine: Affine::translate(Vec2::new(x, y)),
        }
    }

    // Creates a new Transform with translation (Vec2).
    pub fn translate(v: Vec2) -> Self {
        Self {
            affine: Affine::translate(v),
        }
    }

    // Creates a new Transform with rotation (radians).
    pub fn rotate(radians: f64) -> Self {
        Self {
            affine: Affine::rotate_about(radians, Point::new(0.0, 0.0)),
        }
    }

    // Creates a new Transform with a uniform scale.
    pub fn scale(scale: f64) -> Self {
        Self {
            affine: Affine::scale(scale),
        }
    }

    // Creates a new Transform with a non-uniform scale (Vec2).
    pub fn scale_non_uniform(v: Vec2) -> Self {
        Self {
            affine: Affine::scale_non_uniform(v.x, v.y),
        }
    }

    // Adds translation to an already existing Transform.
    pub fn then_translate(&self, v: Vec2) -> Self {
        Self {
            affine: self.affine.then_translate(v),
        }
    }

    // Adds rotation to an already existing Transform.
    pub fn then_rotate(&self, radians: f64) -> Self {
        Self {
            affine: self
                .affine
                .then_rotate_about(radians, self.affine.translation().to_point()),
        }
    }

    // Adds uniform scaling to an already existing Transform.
    pub fn then_scale(&self, scale: f64) -> Self {
        Self {
            affine: self.affine.then_scale(scale),
        }
    }

    // Adds non-uniform scaling to an already existing Transform.
    pub fn then_scale_non_uniform(&self, v: Vec2) -> Self {
        Self {
            affine: self.affine.then_scale_non_uniform(v.x, v.y),
        }
    }

    // Modifies the transform's position.
    pub fn add_translation(&mut self, v: Vec2) {
        self.affine = self.affine.then_translate(v);
    }
}
