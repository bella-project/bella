//! # Bella Engine.
//!
//! **A game engine fully powered by vector graphics.**
//!
//! It combines the power of Bevy's ECS with the rendering and compute shading of Vello. Designed to be light and performant as possible at runtime.

pub mod components;
pub mod input;
pub mod time;
pub mod transforms;

/// This is the entry point of the engine, where it exports all of the tools you and Bella need and manages the root of your program.
pub mod prelude {

    use winit::{
        window::Window,
        event::{
            WindowEvent,
            ElementState,
        },
        dpi::PhysicalSize,
        platform::scancode::PhysicalKeyExtScancode,
        application::ApplicationHandler,
        event_loop::{
            EventLoop,
            ActiveEventLoop,
            ControlFlow,
        },
    };

    use std::sync::Arc;

    #[doc(hidden)]
    pub use winit::keyboard::KeyCode;

    use std::time::Duration;

    use std::num::NonZeroUsize;

    use vello::kurbo::{Affine, Circle, Ellipse, Line, RoundedRect, Stroke};
    use vello::peniko::Color;
    use vello::util::{RenderContext, RenderSurface};
    use vello::{AaConfig, DebugLayers, Renderer, RendererOptions, Scene};

    use vello::wgpu;

    use pollster;

    use std::collections::HashMap;

    use crossbeam_queue::ArrayQueue;

    #[doc(hidden)]
    pub use vello::{kurbo, peniko};

    #[doc(hidden)]
    pub use crate::{
        components::{new_bella_scene, render, BellaScene, BellaShape},
        input::{recieve_inputs, BellaInput},
        time::BellaTime,
        transforms::BellaTransform,
    };

    #[doc(hidden)]
    pub use bevy_ecs::prelude::*;

    /// Helper function that creates a Winit window and returns it (wrapped in an Arc for sharing between threads)
    fn create_winit_window(event_loop: &ActiveEventLoop, title: &str, width: u32, height: u32) -> Arc<Window> {
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

    /// The root of your Bella program.
    pub struct BellaApp<'a> {
        world: World,

        sch_on_start: Schedule,
        sch_on_render: Schedule,
        sch_on_pre_update: Schedule,
        sch_on_update: Schedule,

        title: String,
        width: u32,
        height: u32,

        on_start: bool,
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
    pub struct BellaInstance {
        pub max_scene_id: usize,
        pub scenes: HashMap<usize, Scene>,
    }

    fn bella_instance_reset(mut root: ResMut<BellaInstance>, mut time: ResMut<BellaTime>) {
        time.start_delta();

        for (id, scene) in &mut root.scenes {
            scene.reset();
        }
    }

    impl<'a> ApplicationHandler for BellaApp<'a> {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let RenderState::Suspended(cached_window) = &mut self.state else {
                return;
            };

            // Get the winit window cached in a previous Suspended event or else create a new window
            let window = cached_window
                .take()
                .unwrap_or_else(|| create_winit_window(event_loop, &self.title, self.width, self.height));
    
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

        fn suspended(&mut self, event_loop: &ActiveEventLoop) {
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
                    self.width = size.width;
                    self.height = size.height;

                    self.is_resizing = true;
                    self.new_resize = true;
                },

                WindowEvent::KeyboardInput { event, .. } => {

                    let input = self.world.get_resource::<BellaInput>().unwrap();

                    match event.state {
                        ElementState::Pressed => {
                            input.set_key_down(event.physical_key.to_scancode().unwrap());
                        },
                        ElementState::Released => {
                            input.set_key_up(event.physical_key.to_scancode().unwrap());
                        },
                    }
                },
    
                // This is where all the rendering happens
                WindowEvent::RedrawRequested => {

                    if self.is_resizing {
                        render_state.window.request_redraw();
                        self.is_resizing = false;
                        return;
                    }

                    if self.on_start {
                        self.sch_on_start.run(&mut self.world);
                        self.on_start = false;
                    }

                    // Get the RenderSurface (surface + config)
                    let mut surface = &mut render_state.surface;

                    let width = self.width;
                    let height = self.height;

                    // This is a fix to try to smooth resizing on Windows.
                    if self.new_resize {
                        self.context
                            .resize_surface(&mut surface, width, height);
                        self.new_resize = false;
                    }

                    let device_handle = &self.context.devices[surface.dev_id];

                    self.main_scene.reset();

                    self.sch_on_render.run(&mut self.world);

                    let root = self.world.get_resource::<BellaInstance>().unwrap();
        
                    for (id, scene) in &root.scenes {
                        self.main_scene.append(&scene, None);
                    }

                    let surface_texture = surface
                        .surface
                        .get_current_texture()
                        .expect("failed to get surface texture");
        
                    self.renderers[surface.dev_id]
                        .as_mut()
                        .unwrap()
                        .render_to_surface(
                            &device_handle.device,
                            &device_handle.queue,
                            &self.main_scene,
                            &surface_texture,
                            &vello::RenderParams {
                                base_color: Color::BLACK, // Background color
                                width,
                                height,
                                antialiasing_method: AaConfig::Msaa16,
                                debug: DebugLayers::none(),
                            },
                        )
                        .expect("failed to render to surface");

                    surface_texture.present();

                    device_handle.device.poll(wgpu::Maintain::Poll);

                    self.sch_on_pre_update.run(&mut self.world);

                    self.sch_on_update.run(&mut self.world);

                    render_state.window.request_redraw();
                },
                _ => {},
            }
        }
    }

    impl BellaApp<'_> {
        /// Creates a new [`BellaApp`] with a window ready to go.
        /// `title` sets the title of the window, `width` and `height` set the resolution.
        pub fn new(title: &str, width: u32, height: u32) -> Self {

            let mut world = World::new();

            world.insert_resource(BellaInstance::default());
            world.insert_resource(BellaTime::new());
            world.insert_resource(BellaInput::default());

            let mut sch_on_render = Schedule::default();

            sch_on_render.add_systems(bella_instance_reset);
            sch_on_render.add_systems(render.after(bella_instance_reset));

            let mut sch_on_pre_update = Schedule::default();

            sch_on_pre_update.add_systems(recieve_inputs);

            Self {
                world: world,

                sch_on_start: Schedule::default(),
                sch_on_render: sch_on_render,
                sch_on_pre_update: sch_on_pre_update,
                sch_on_update: Schedule::default(),

                title: title.to_string(),
                width: width,
                height: height,

                on_start: true,
                is_resizing: false,
                new_resize: false,

                context: RenderContext::new(),
                renderers: vec![],
                state: RenderState::Suspended(None),
                main_scene: Scene::new(),
            }
        }

        /// Adds a system that'll be executed on the first frame of your program.
        pub fn on_start<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
            self.sch_on_start.add_systems(systems);
            self
        }

        /// Adds a system that'll be executed in the render loop.
        /// This is used for rendering the Vello Shapes, for example.
        pub fn on_render<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
            self.sch_on_render.add_systems(systems);
            self
        }

        /// Adds a system that'll be executed every frame.
        /// This is where you usually run your game logic, like inputs, player controllers, etc.
        pub fn on_update<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
            self.sch_on_update.add_systems(systems);
            self
        }

        /// Runs your [`BellaApp`].
        pub fn run(&mut self) {
            let event_loop = EventLoop::new().unwrap();
            event_loop
                .run_app(self)
                .expect("Couldn't run event loop");
        }
    }
}
