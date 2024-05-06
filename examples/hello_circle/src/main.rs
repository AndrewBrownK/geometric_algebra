use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::io::Read;

use naga_oil::compose::{ComposableModuleDescriptor, Composer, NagaModuleDescriptor};
use naga_oil::prune::PartReq;
use wgpu::{Instance, InstanceDescriptor, SurfaceTargetUnsafe};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{
    Window, WindowId,
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

struct App {
    window: Option<Window>,
    handle_redraw: Box<dyn FnMut(&ActiveEventLoop, WindowId, WindowEvent)>
}
impl App {
    fn new() -> Self {
        App {
            window: None,
            handle_redraw: Box::new(|_, _, _| {})
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.start_wgpu_if_necessary(&event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.handle_redraw.as_mut()(event_loop, id, event);
                self.window.as_ref().unwrap().request_redraw()
            },
            _ => (),
        }
    }
}

impl App {
    fn start_wgpu_if_necessary(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        pollster::block_on(self.initiate(event_loop));
    }

    async fn initiate(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Default::default()).unwrap();
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = unsafe {
            instance.create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap()).unwrap()
        };
        let adapter = instance.request_adapter(&Default::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();
        let window_size = window.inner_size();


        let wgsl_entry_path = "examples/src/shader.wgsl";
        let wgsl_entry = fs::read_to_string(wgsl_entry_path).unwrap();
        let naga_module_descriptor = NagaModuleDescriptor {
            source: wgsl_entry.as_str(),
            file_path: wgsl_entry_path,
            ..Default::default()
        };

        let naga_module = cga3d_min::shaders::wgsl_compose_with_entrypoints(naga_module_descriptor).unwrap();



        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            // source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            source: wgpu::ShaderSource::Naga(Cow::Owned(naga_module)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let config = surface
            .get_default_config(&adapter, window_size.width, window_size.height)
            .unwrap();
        surface.configure(&device, &config);

        window.request_redraw();
        self.window = Some(window);

        // TODO render a circle instead of a triangle
        self.handle_redraw = Box::new(move |event_loop, id, event| {
            let frame = surface
                .get_current_texture()
                .expect("Failed to acquire next swap chain texture");
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: None,
                });
            {
                let mut rpass =
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                rpass.set_pipeline(&render_pipeline);
                rpass.draw(0..3, 0..1);
            }

            queue.submit(Some(encoder.finish()));
            frame.present();
        });
    }
}