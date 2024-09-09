//! Handles the inputs (keyboard, joystick, mouse, etc.) of your [`BellaApp`].

use crate::prelude::*;

use winit::keyboard::PhysicalKey::Code;
/// The representation of a key from your keyboard, in form of a struct. Powered by [`winit`].
pub use winit::keyboard::{KeyCode, PhysicalKey};
use winit::platform::scancode::PhysicalKeyExtScancode;

use crossbeam_queue::SegQueue;

/// The Resource that takes care of the communication of [`winit`]'s inputs coming from [`BellaApp::window_main`]'s event loop.
///
/// - `key_down_queue` is a [`crossbeam_queue::SegQueue`] where all of the keys that are down are sent so [`recieve_inputs`] can detect them.
/// - `key_up_queue` is the same, but for keys that are up.
///
/// - `key_down` is a vector that contains all of the "key down"'s derected by [`recieve_inputs`].
/// - `key_up` is a vector that contains all of the "key up"'s derected by [`recieve_inputs`].
/// - `key_press` is a vector that contains all of the keys currently being pressed, derected by [`recieve_inputs`].
#[derive(Resource, Default)]
pub struct BellaInput {
    key_down_queue: SegQueue<u32>,
    key_up_queue: SegQueue<u32>,

    key_down: Vec<u32>,
    key_up: Vec<u32>,
    key_press: Vec<u32>,
}

/// The logic that absorbs all of the information coming from [`BellaInput`]'s queues, so it can be used later for your app's systems.
pub fn recieve_inputs(mut input: ResMut<BellaInput>) {
    input.key_down.clear();
    input.key_up.clear();

    while !input.key_down_queue.is_empty() {
        let k = input.key_down_queue.pop().unwrap();
        input.key_down.push(k);

        let mut is_key_already_pressed: bool = false;
        for kp in &input.key_press {
            if *kp == k {
                is_key_already_pressed = true;
            }
        }

        if !is_key_already_pressed {
            input.key_press.push(k);
        }
    }

    while !input.key_up_queue.is_empty() {
        let k = input.key_up_queue.pop().unwrap();
        input.key_up.push(k);

        input.key_press.retain(|x| *x != k);
    }
}

fn get_keycode_from_physical_key(pk: PhysicalKey) -> KeyCode {
    match pk {
        Code(kc) => kc,
        _ => panic!("KeyCode not found!"),
    }
}

impl BellaInput {
    /// Sends a key down to the `key_down_queue`. Currently used in [`BellaApp::window_main`].
    pub fn set_key_down(&self, key: u32) {
        self.key_down_queue.push(key);
    }

    /// Sends a key up to the `key_up_queue`. Currently used in [`BellaApp::window_main`].
    pub fn set_key_up(&self, key: u32) {
        self.key_up_queue.push(key);
    }

    /// Checks if a key is down.
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        for k in &self.key_down {
            if get_keycode_from_physical_key(KeyCode::from_scancode(*k)) == key {
                return true;
            }
        }

        false
    }

    /// Checks if a key is up.
    pub fn is_key_up(&self, key: KeyCode) -> bool {
        for k in &self.key_up {
            if get_keycode_from_physical_key(KeyCode::from_scancode(*k)) == key {
                return true;
            }
        }

        false
    }

    /// Checks if a key is currently being pressed.
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        for k in &self.key_press {
            if get_keycode_from_physical_key(KeyCode::from_scancode(*k)) == key {
                return true;
            }
        }

        false
    }
}
