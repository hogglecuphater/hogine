use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::rendering::rendering_backend::{DisplaySize, RenderBackend};

pub mod rendering_backend;
pub mod wgpu;

pub mod graphics3d;

pub struct Renderer<T: RenderBackend> {
    render_backend: T,
}

impl<T: RenderBackend> Renderer<T> {
    pub fn new(window_handle: RawWindowHandle, display_handle: RawDisplayHandle, size: DisplaySize) -> Self {
        Self { render_backend: T::create(window_handle, display_handle, size) }
    }

    pub fn resize(&mut self, size: DisplaySize) {
        self.render_backend.resize(size);
    }

    pub fn render_test_frame(&mut self) {
        self.render_backend.render_test_frame();
    }
}