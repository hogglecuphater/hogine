use raw_window_handle::{RawWindowHandle, RawDisplayHandle};
use wgpu::{RenderPipeline, util::DeviceExt};

use crate::rendering::{rendering_backend::{DisplaySize, RenderBackend}, wgpu::vertex::{GpuVertex, VERTICES}};

mod vertex;
mod shaders;

pub enum WgpuRenderError {
    FailedToGetCurrentSurfaceTexture,
}

pub struct WgpuRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl WgpuRenderer {
    pub fn new(window_handle: RawWindowHandle, display_handle: RawDisplayHandle, size: DisplaySize) -> Self {
        let instance = wgpu::Instance::default();

        let surface = unsafe {
            let target = wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_window_handle: window_handle,
                raw_display_handle: Some(display_handle),
            };
            instance.create_surface_unsafe(target).unwrap()
        };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("Failed to find a suitable GPU adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(), // Added in recent wgpu versions
            trace: wgpu::Trace::Off, // Added in recent wgpu versions
        }))
        .expect("Failed to request device");

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface not supported by adapter");
        surface.configure(&device, &config);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shaders::SHADER.into()),
        });

        let render_pipeline = Self::create_render_pipeline(&device, &config, &shader);

        Self { surface, device, queue, config, vertex_buffer, render_pipeline }
    }

    fn create_render_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, shader:&wgpu::ShaderModule) -> RenderPipeline {
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(GpuVertex::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        })
    }

    pub fn resize(&mut self, size: DisplaySize) {
        if size.width > 0 && size.height > 0 {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render_frame(&self) -> Result<(), WgpuRenderError> {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            _ => {
                return Err(WgpuRenderError::FailedToGetCurrentSurfaceTexture);
            }
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                ..Default::default()
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..vertex::VERTICES.len() as u32, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(frame);

        Ok(())
    }
}

impl RenderBackend for WgpuRenderer {
    fn create(window_handle: RawWindowHandle, display_handle: RawDisplayHandle, size: DisplaySize) -> Self {
        Self::new(window_handle, display_handle, size)
    }

    fn resize(&mut self, size: DisplaySize) {
        self.resize(size);
    }

    fn render_test_frame(&mut self) {
        self.render_frame();
    }
}