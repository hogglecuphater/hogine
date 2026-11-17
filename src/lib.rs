use glam::UVec2;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::rendering::Renderer;
use crate::rendering::rendering_backend::DisplaySize;
use crate::rendering::wgpu::WgpuRenderer;

mod rendering;

#[derive(Debug)]
enum UserEvent {

}

#[derive(Default)]
struct App {
    window: Option<Window>,
    proxy: Option<EventLoopProxy<UserEvent>>,
    renderer: Option<Renderer<WgpuRenderer>>,
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        // 1. Create the winit window
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Pure Rust Winit + Wgpu Window")
                    .with_inner_size(winit::dpi::LogicalSize::new(800, 600)),
            )
            .expect("Failed to create window");

        // Kick off the initial redraw so Wayland maps the window surface
        window.request_redraw();

        let window_handle = window.window_handle().unwrap().as_raw();
        let display_handle = window.display_handle().unwrap().as_raw();
        let size = window.inner_size();

        self.window = Some(window);
        self.renderer = Some(Renderer::new(window_handle, display_handle, DisplaySize::new(size.width, size.height)));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(DisplaySize::new(new_size.width, new_size.height));
                }
            }
            WindowEvent::RedrawRequested => {
                let window = match &self.window {
                    Some(w) => w,
                    None => return,
                };

                if let Some(renderer) = &mut self.renderer {
                    renderer.render_test_frame();
                }

                // Request the next frame continuously
                window.request_redraw();
            }
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        
    }
}

pub fn run() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}