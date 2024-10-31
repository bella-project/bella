//! Everything related to translation, rotation, scaling and any kind of transformation.

use bevy_ecs::prelude::*;

use vello::peniko::kurbo::Affine;

/// Describes the position, rotation, scale and any kind of transformation of an entity. This is a translation layer between Bella and [`vello::kurbo`]'s [`Affine`].
#[derive(Component)]
pub struct Transform {
    pub affine: Affine,
}

impl Transform {
    pub fn new(a: Affine) -> Self {
        Self { affine: a }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Affine::IDENTITY)
    }
}
