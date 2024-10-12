//! Everything that's related to time.

use crate::prelude::*;

use std::time::{Duration, Instant};

/// The Resource that manages everything related to time in a [`BellaApp`].
#[derive(Resource, Debug, Copy, Clone)]
pub struct Time<T: Default = ()> {
    context: T,
    wrap_period: Duration,
    delta: Duration,
    delta_seconds: f64,
}

impl<T: Default> Time<T> {
    const DEFAULT_WRAP_PERIOD: Duration = Duration::from_secs(3600);

    /// Creates a new clock from a specific context, starting from zero.
    pub fn new_with(context: T) -> Self {
        Self {
            context,
            ..Default::default()
        }
    }

    #[inline]
    pub fn context(&self) -> &T {
        &self.context
    }

    #[inline]
    pub fn context_mut(&mut self) -> &mut T {
        &mut self.context
    }

    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn as_generic(&self) -> Time<()> {
        Time {
            context: (),
            wrap_period: self.wrap_period,
            delta: self.delta,
            delta_seconds: self.delta_seconds,
        }
    }

    pub fn advance_by(&mut self, delta: Duration) {
        self.delta = delta;
        self.delta_seconds = self.delta.as_secs_f64();
    }

    /// Gets the Delta Time in seconds.
    pub fn delta_seconds(&self) -> f64 {
        self.delta_seconds
    }
}

impl<T: Default> Default for Time<T> {
    fn default() -> Self {
        Self {
            context: Default::default(),
            wrap_period: Self::DEFAULT_WRAP_PERIOD,
            delta: Duration::ZERO,
            delta_seconds: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Virtual {
    max_delta: Duration,
    paused: bool,
    relative_speed: f64,
    effective_speed: f64,
}

impl Default for Virtual {
    fn default() -> Self {
        Self {
            max_delta: Time::<Virtual>::DEFAULT_MAX_DELTA,
            paused: false,
            relative_speed: 1.0,
            effective_speed: 1.0,
        }
    }
}

impl Time<Virtual> {
    const DEFAULT_MAX_DELTA: Duration = Duration::from_millis(250);

    fn advance_with_raw_delta(&mut self, raw_delta: Duration) {
        let max_delta = self.context().max_delta;
        let clamped_delta = if raw_delta > max_delta {
            // debug!(
            //     "delta time larger than maximum delta, clamping delta to {:?} and skipping {:?}",
            //     max_delta,
            //     raw_delta - max_delta
            // );
            max_delta
        } else {
            raw_delta
        };
        let effective_speed = if self.context().paused {
            0.0
        } else {
            self.context().relative_speed
        };
        let delta = if effective_speed != 1.0 {
            clamped_delta.mul_f64(effective_speed)
        } else {
            // avoid rounding when at normal speed
            clamped_delta
        };
        self.context_mut().effective_speed = effective_speed;
        self.advance_by(delta);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Real {
    #[allow(dead_code)]
    startup: Instant,
    first_update: Option<Instant>,
    last_update: Option<Instant>,
}

impl Default for Real {
    fn default() -> Self {
        Self {
            startup: Instant::now(),
            first_update: None,
            last_update: None,
        }
    }
}

impl Time<Real> {
    pub fn new(startup: Instant) -> Self {
        Self::new_with(Real {
            startup,
            ..Default::default()
        })
    }

    pub fn update_with_instant(&mut self, instant: Instant) {
        let Some(last_update) = self.context().last_update else {
            let context = self.context_mut();
            context.first_update = Some(instant);
            context.last_update = Some(instant);
            return;
        };
        let delta = instant - last_update;
        self.advance_by(delta);
        self.context_mut().last_update = Some(instant);
    }
}

pub fn time_system(
    mut real_time: ResMut<Time<Real>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut time: ResMut<Time>,
) {
    let new_time = Instant::now();

    real_time.update_with_instant(new_time);

    update_time(&mut time, &mut virtual_time, &real_time);
}

pub fn update_time(current: &mut Time, virt: &mut Time<Virtual>, real: &Time<Real>) {
    let raw_delta = real.delta();
    virt.advance_with_raw_delta(raw_delta);
    *current = virt.as_generic();
}
