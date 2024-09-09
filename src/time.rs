//! Everything that's related to time.

use crate::prelude::*;

use std::time::Instant;

/// The Resource that manages everything related to time in a [`BellaApp`].
#[derive(Resource)]
pub struct BellaTime {
    delta: Instant,
}

impl BellaTime {
    /// Instantiates the Resource. Currently used in [`BellaApp`]'s creation.
    pub fn new() -> Self {
        Self {
            delta: Instant::now(),
        }
    }

    /// Starts/Resets the Delta Time. Currently used in the reset of [`BellaInstance`].
    pub fn start_delta(&mut self) {
        self.delta = Instant::now();
    }

    /// Gets the Delta Time in seconds.
    pub fn delta_seconds(&self) -> f64 {
        self.delta.elapsed().as_secs_f64()
    }
}

impl Default for BellaTime {
    fn default() -> Self {
        Self::new()
    }
}
