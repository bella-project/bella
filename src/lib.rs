//! # Bella Engine.
//!
//! **A game engine fully powered by vector graphics.**
//!
//! It combines the power of Bevy's ECS with the rendering and compute shading of Vello. Designed to be light and performant as possible at runtime.

pub mod input;
pub mod time;
pub mod transforms;

pub extern crate interpoli;

/// This is the entry point of the engine, where it exports all of the tools you and Bella need and manages the root of your program.
pub mod prelude {

    use winit::{
        application::ApplicationHandler,
        dpi::PhysicalSize,
        event::{ElementState, WindowEvent},
        event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
        platform::scancode::PhysicalKeyExtScancode,
        window::Window,
    };

    use std::sync::Arc;

    #[doc(hidden)]
    pub use winit::keyboard::KeyCode;

    use std::num::NonZeroUsize;

    use vello::peniko::Color;
    use vello::util::{RenderContext, RenderSurface};
    use vello::{AaConfig, Renderer, RendererOptions, Scene};

    use vello::wgpu;

    use pollster;

    use std::collections::HashMap;

    pub use interpoli::timeline;
    pub use interpoli::{tcode_full, tcode_hms, tcode_hmsf, tcode_hmsf_framerate};

    #[doc(hidden)]
    pub use vello::{kurbo, peniko};

    #[doc(hidden)]
    pub use crate::{
        input::{recieve_inputs, Input},
        time::{time_system, Real, Time, Virtual},
        transforms::Transform,
    };

    #[doc(hidden)]
    pub use bevy_ecs::prelude::*;

    /// Helper function that creates a Winit window and returns it (wrapped in an Arc for sharing between threads)
    fn create_winit_window(
        event_loop: &ActiveEventLoop,
        title: &str,
        width: u32,
        height: u32,
    ) -> Arc<Window> {
        let attr = Window::default_attributes()
            .with_inner_size(PhysicalSize::new(width, height))
            .with_resizable(true)
            .with_title(title);
        Arc::new(event_loop.create_window(attr).unwrap())
    }

    fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface) -> Renderer {
        Renderer::new(
            &render_cx.devices[surface.dev_id].device,
            RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: NonZeroUsize::new(1),
            },
        )
        .expect("Couldn't create renderer")
    }

    // Simple struct to hold the state of the renderer
    pub struct ActiveRenderState<'s> {
        // The fields MUST be in this order, so that the surface is dropped before the window
        surface: RenderSurface<'s>,
        window: Arc<Window>,
    }

    enum RenderState<'s> {
        Active(ActiveRenderState<'s>),
        // Cache a window so that it can be reused when the app is resumed after being suspended
        Suspended(Option<Arc<Window>>),
    }

    pub struct BellaWorld {
        pub main: World,

        pub sch_on_start: Schedule,
        pub sch_on_first: Schedule,
        pub sch_on_draw: Schedule,
        pub sch_on_pre_update: Schedule,
        pub sch_on_update: Schedule,
        pub sch_on_last: Schedule,

        on_start: bool,
    }

    impl Default for BellaWorld {
        fn default() -> Self {
            Self::new()
        }
    }

    impl BellaWorld {
        pub fn new() -> Self {
            let mut world = World::new();

            world.insert_resource(Instance::default());
            world.insert_resource(Time::new_with(()));
            world.insert_resource(Time::new_with(Virtual::default()));
            world.insert_resource(Time::new_with(Real::default()));
            world.insert_resource(Input::default());

            let mut sch_on_first = Schedule::default();

            sch_on_first.add_systems(time_system);
            sch_on_first.add_systems(bella_instance_reset);

            let sch_on_draw = Schedule::default();
            let mut sch_on_pre_update = Schedule::default();

            sch_on_pre_update.add_systems(recieve_inputs);

            Self {
                main: world,
                sch_on_start: Schedule::default(),
                sch_on_first,
                sch_on_draw,
                sch_on_pre_update,
                sch_on_update: Schedule::default(),
                sch_on_last: Schedule::default(),
                on_start: true,
            }
        }
    }

    /// The root of your Bella program.
    pub struct App<'a> {
        worlds: Vec<BellaWorld>,

        title: String,
        width: u32,
        height: u32,

        new_resize: bool,
        is_resizing: bool,

        context: RenderContext,
        renderers: Vec<Option<Renderer>>,
        state: RenderState<'a>,
        main_scene: Scene,
    }

    /// The root of all of your `BellaScene`'s, which are stored and sent to the CPU/GPU.
    ///
    /// - `max_scene_id` keeps track of the last scene ID. This is used as a counter which increases each time you call [`new_bella_scene`].
    /// - `scenes` is a [`HashMap`] that stores all of the Scenes internally, all of them containing the unique IDs that have been assigned by [`new_bella_scene`] via `max_scene_id`.
    #[derive(Resource, Default)]
    pub struct Instance {
        pub max_scene_id: usize,
        pub scenes: HashMap<usize, Scene>,
        pub scene_names: HashMap<String, usize>,
    }

    impl Instance {
        pub fn new_scene(&mut self, name: &str) -> Option<&mut Scene> {
            self.max_scene_id += 1;
            self.scenes.insert(self.max_scene_id, Scene::new());
            self.scene_names.insert(name.to_string(), self.max_scene_id);

            self.scenes.get_mut(&self.max_scene_id)
        }

        pub fn get_scene(&mut self, name: &str) -> Option<&mut Scene> {
            let ptr = self.scene_names.get(name);

            match ptr {
                Some(p) => self.scenes.get_mut(p),
                None => None,
            }
        }
    }

    fn bella_instance_reset(mut root: ResMut<Instance>) {
        #[allow(clippy::for_kv_map)]
        for (_id, scene) in &mut root.scenes {
            scene.reset();
        }
    }

    impl<'a> ApplicationHandler for App<'a> {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let RenderState::Suspended(cached_window) = &mut self.state else {
                return;
            };

            // Get the winit window cached in a previous Suspended event or else create a new window
            let window = cached_window.take().unwrap_or_else(|| {
                create_winit_window(event_loop, &self.title, self.width, self.height)
            });

            // Create a vello Surface
            let size = window.inner_size();
            let surface_future = self.context.create_surface(
                window.clone(),
                size.width,
                size.height,
                wgpu::PresentMode::AutoVsync,
            );
            let surface = pollster::block_on(surface_future).expect("Error creating surface");

            // Create a vello Renderer for the surface (using its device id)
            self.renderers
                .resize_with(self.context.devices.len(), || None);
            self.renderers[surface.dev_id]
                .get_or_insert_with(|| create_vello_renderer(&self.context, &surface));

            // Save the Window and Surface to a state variable
            self.state = RenderState::Active(ActiveRenderState { window, surface });

            event_loop.set_control_flow(ControlFlow::Wait);
        }

        fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
            if let RenderState::Active(state) = &self.state {
                self.state = RenderState::Suspended(Some(state.window.clone()));
            }
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
            // Ignore the event (return from the function) if
            //   - we have no render_state
            //   - OR the window id of the event doesn't match the window id of our render_state
            //
            // Else extract a mutable reference to the render state from its containing option for use below
            let render_state = match &mut self.state {
                RenderState::Active(state) if state.window.id() == window_id => state,
                _ => return,
            };

            match event {
                // Exit the event loop when a close is requested (e.g. window's close button is pressed)
                WindowEvent::CloseRequested => event_loop.exit(),

                // Resize the surface when the window is resized
                WindowEvent::Resized(size) => {
                    if size.width == 0 && size.height == 0 {
                        return;
                    }

                    self.width = size.width;
                    self.height = size.height;

                    self.is_resizing = true;
                    self.new_resize = true;
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    for w in &self.worlds {
                        let input = w.main.get_resource::<Input>().unwrap();

                        match event.state {
                            ElementState::Pressed => {
                                input.set_key_down(event.physical_key.to_scancode().unwrap());
                            }
                            ElementState::Released => {
                                input.set_key_up(event.physical_key.to_scancode().unwrap());
                            }
                        }
                    }
                }

                // This is where all the rendering happens
                WindowEvent::RedrawRequested => {
                    if self.is_resizing {
                        render_state.window.request_redraw();
                        self.is_resizing = false;
                        return;
                    }

                    // Get the RenderSurface (surface + config)
                    let surface = &mut render_state.surface;

                    let width = self.width;
                    let height = self.height;

                    // This is a fix to try to smooth resizing on Windows.
                    if self.new_resize {
                        self.context.resize_surface(surface, width, height);
                        self.new_resize = false;
                    }

                    let device_handle = &self.context.devices[surface.dev_id];

                    self.main_scene.reset();

                    let surface_texture = surface
                        .surface
                        .get_current_texture()
                        .expect("failed to get surface texture");

                    let mut first_draw_call: bool = true;

                    for w in &mut self.worlds {
                        if w.on_start {
                            w.sch_on_start.run(&mut w.main);
                            w.on_start = false;
                        }

                        w.sch_on_first.run(&mut w.main);

                        w.sch_on_draw.run(&mut w.main);

                        let root = w.main.get_resource::<Instance>().unwrap();

                        #[allow(clippy::for_kv_map)]
                        for (_id, scene) in &root.scenes {
                            self.main_scene.append(scene, None);
                        }

                        self.renderers[surface.dev_id]
                            .as_mut()
                            .unwrap()
                            .render_to_surface(
                                &device_handle.device,
                                &device_handle.queue,
                                &self.main_scene,
                                &surface_texture,
                                &vello::RenderParams {
                                    base_color: if first_draw_call {
                                        first_draw_call = false;
                                        Color::BLACK
                                    } else {
                                        Color::TRANSPARENT
                                    }, // Background color
                                    width,
                                    height,
                                    antialiasing_method: AaConfig::Msaa16,
                                },
                            )
                            .expect("failed to render to surface");

                        w.sch_on_pre_update.run(&mut w.main);

                        w.sch_on_update.run(&mut w.main);

                        w.sch_on_last.run(&mut w.main);
                    }

                    surface_texture.present();

                    device_handle.device.poll(wgpu::Maintain::Poll);

                    render_state.window.request_redraw();
                }
                _ => {}
            }
        }
    }

    impl App<'_> {
        /// Creates a new [`App`] with a window ready to go.
        /// `title` sets the title of the window, `width` and `height` set the resolution.
        pub fn new(title: &str, width: u32, height: u32) -> Self {
            Self {
                worlds: vec![],

                title: title.to_string(),
                width,
                height,

                is_resizing: false,
                new_resize: false,

                context: RenderContext::new(),
                renderers: vec![],
                state: RenderState::Suspended(None),
                main_scene: Scene::new(),
            }
        }

        /// Creates a new world.
        pub fn new_world(&mut self) -> &mut Self {
            self.worlds.push(BellaWorld::new());
            self
        }

        /// Adds a system that'll be executed on the first frame of your world.
        pub fn on_start<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
            self.worlds
                .last_mut()
                .unwrap()
                .sch_on_start
                .add_systems(systems);
            self
        }

        /// Adds a system that'll be executed in the render loop.
        /// This is used for rendering the Vello Shapes, for example.
        pub fn on_draw<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
            self.worlds
                .last_mut()
                .unwrap()
                .sch_on_draw
                .add_systems(systems);
            self
        }

        /// Adds a system that'll be executed every frame.
        /// This is where you usually run your game logic, like inputs, player controllers, etc.
        pub fn on_update<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
            self.worlds
                .last_mut()
                .unwrap()
                .sch_on_update
                .add_systems(systems);
            self
        }

        /// Runs your [`App`].
        pub fn run(&mut self) {
            let event_loop = EventLoop::new().unwrap();
            event_loop.run_app(self).expect("Couldn't run event loop");
        }
    }
}
