use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub struct DisplaySize {
    pub width: u32,
    pub height: u32,
}

impl DisplaySize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Into<(u32, u32)> for DisplaySize {
    fn into(self) -> (u32, u32) {
        (self.width, self.height)
    }
}


pub trait RenderBackend {
    fn create(window_handle: RawWindowHandle, display_handle: RawDisplayHandle, size: DisplaySize) -> Self;

    fn resize(&mut self, size: DisplaySize);

    fn render_test_frame(&mut self);
}