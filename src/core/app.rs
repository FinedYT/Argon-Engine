use crate::renderer::context::RenderContext;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    render_context: Option<RenderContext>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            println!("window is none");

            let attributes = Window::default_attributes()
                .with_title("Argon Engine")
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));

            let window = event_loop.create_window(attributes).unwrap();
            let window = Arc::new(window);
            let render_context = pollster::block_on(RenderContext::new(window.clone()));
            self.render_context = Some(render_context);
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                if let Some(ctx) = &mut self.render_context {
                    ctx.resize(physical_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(ctx) = &mut self.render_context {
                    ctx.render();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
