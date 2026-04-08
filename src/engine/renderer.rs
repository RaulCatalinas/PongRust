use std::sync::Arc;

use tao::window::Window;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance,
    InstanceDescriptor, LoadOp, Operations, PowerPreference, PresentMode, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    SurfaceConfiguration, SurfaceError, SurfaceTarget, TextureUsages, TextureViewDescriptor,
};

pub struct Renderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    window: Arc<Window>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        // 1. Instance — detecta backends disponibles
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // 2. Surface — vinculada a la ventana de tao
        let surface = instance
            .create_surface(SurfaceTarget::Window(Box::new(Arc::clone(&window))))
            .unwrap();

        // 3. Adapter — elige la GPU más adecuada
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // 4. Device + Queue
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor::default(),
                None, // trace path
            )
            .await
            .unwrap();

        // 5. SurfaceConfig — formato y tamaño de la surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: PresentMode::Fifo, // VSync
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            window,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        // Pide el siguiente frame
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        // Graba comandos GPU
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Render pass — por ahora solo limpia la pantalla con un color
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
