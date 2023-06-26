use std::sync::Arc;

use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::app::App;

struct Renderer {
    app: Arc<App>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    fn new(app: Arc<App>) -> Result<Self> {
        // let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        // let surface = unsafe { instance.create_surface(&window) }.unwrap();
        // let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        //     power_preference: wgpu::PowerPreference::default(),
        //     compatible_surface: Some(&surface),
        //     force_fallback_adapter: false,
        // }))
        // .unwrap();

        // let (device, queue) = pollster::block_on(adapter.request_device(
        //     &wgpu::DeviceDescriptor {
        //         label: None,
        //         features: wgpu::Features::empty(),
        //         limits: wgpu::Limits::default(),
        //     },
        //     None,
        // ))
        // .unwrap();

        // let texture_size = wgpu::Extent3d {
        //     width: width as u32,
        //     height: height as u32,
        //     depth_or_array_layers: 1,
        // };
        // let texture = device.create_texture(&wgpu::TextureDescriptor {
        //     size: texture_size,
        //     mip_level_count: 1,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     // Most images are stored using sRGB so we need to reflect that here.
        //     format: wgpu::TextureFormat::Rgba8UnormSrgb,
        //     // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
        //     // COPY_DST means that we want to copy data to this texture
        //     usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        //     label: Some("texture"),
        //     // This is the same as with the SurfaceConfig. It
        //     // specifies what texture formats can be used to
        //     // create TextureViews for this texture. The base
        //     // texture format (Rgba8UnormSrgb in this case) is
        //     // always supported. Note that using a different
        //     // texture format is not supported on the WebGL2
        //     // backend.
        //     view_formats: &[],
        // });

        // let frame = app.decoder().next_frame().unwrap().unwrap();
        // let frame = frame
        //     .chunks(3)
        //     .flat_map(|p| vec![p[0], p[1], p[2], 255])
        //     .collect::<Vec<_>>();

        // queue.write_texture(
        //     // Tells wgpu where to copy the pixel data
        //     wgpu::ImageCopyTexture {
        //         texture: &texture,
        //         mip_level: 0,
        //         origin: wgpu::Origin3d::ZERO,
        //         aspect: wgpu::TextureAspect::All,
        //     },
        //     // The actual pixel data
        //     &frame,
        //     // The layout of the texture
        //     wgpu::ImageDataLayout {
        //         offset: 0,
        //         bytes_per_row: Some(4 * width as u32),
        //         rows_per_image: Some(height as u32),
        //     },
        //     texture_size,
        // );

        // let diffuse_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        //     address_mode_u: wgpu::AddressMode::ClampToEdge,
        //     address_mode_v: wgpu::AddressMode::ClampToEdge,
        //     address_mode_w: wgpu::AddressMode::ClampToEdge,
        //     mag_filter: wgpu::FilterMode::Linear,
        //     min_filter: wgpu::FilterMode::Nearest,
        //     mipmap_filter: wgpu::FilterMode::Nearest,
        //     ..Default::default()
        // });

        // let size = window.inner_size();
        // surface.configure(
        //     &device,
        //     &surface
        //         .get_default_config(&adapter, size.width, size.height)
        //         .unwrap(),
        // );

        todo!()
    }

    pub fn render(&self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let color = self.app.args.background.unwrap_or([0, 0, 0]);
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: color[0] as f64 / 255.0,
                            g: color[1] as f64 / 255.0,
                            b: color[2] as f64 / 255.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

pub fn init(app: Arc<App>) -> Result<()> {
    let (width, height) = app.decoder().dimensions();

    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("video-presenter \u{2013} {}", app.video_name()))
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)?;

    let renderer = Renderer::new(app.clone())?;

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            window.request_redraw();
        }

        if let Event::RedrawRequested(_) = event {
            renderer.render();
        }
    });
}
