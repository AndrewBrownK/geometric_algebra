use naga::ShaderStage;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Debug;
use std::io::Read;
use std::{fs, thread};

use naga_oil::compose::{NagaModuleDescriptor, ShaderType};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroupDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BufferUsages, Instance, InstanceDescriptor, SurfaceTargetUnsafe};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

struct App {
    window: Option<Window>,
    handle_redraw: Box<dyn FnMut(&ActiveEventLoop, WindowId, WindowEvent)>,
    size: PhysicalSize<u32>,
}
impl App {
    fn new() -> Self {
        App {
            window: None,
            handle_redraw: Box::new(|_, _, _| {}),
            size: PhysicalSize { width: 1u32, height: 1u32 },
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
            }
            WindowEvent::Resized(new_size) => {
                self.size = new_size;
            }
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
        let surface = unsafe { instance.create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap()).unwrap() };
        let adapter = instance.request_adapter(&Default::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();
        let window_size = window.inner_size();
        self.size = window_size.clone();

        let (tx, mut rx) = std::sync::mpsc::channel();
        thread::Builder::new()
            .name("glsl composition".to_string())
            // glsl composition uses recursion and needs larger stack size
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let glsl_frag_entry_path = "examples/broken_glsl/hello_circle_glsl/src/shader.frag.glsl";
                let glsl_entry = fs::read_to_string(glsl_frag_entry_path).unwrap();
                let naga_module_descriptor = NagaModuleDescriptor {
                    source: glsl_entry.as_str(),
                    file_path: glsl_frag_entry_path,
                    shader_type: ShaderType::GlslFragment,
                    ..Default::default()
                };
                let naga_module = cga3d_min::shaders::glsl_compose_with_entrypoints(naga_module_descriptor).unwrap();
                tx.send(naga_module).expect("must tx naga_module successfully");
            })
            .expect("We need multithreading, in order to get a larger stack size, in order to compose wgsl");
        let naga_module = rx.recv().expect("Need glsl naga module");

        let glsl_vert_shader = include_str!("shader.vert.glsl");

        // Load the shaders from disk
        let vert_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Glsl {
                shader: Cow::Owned(glsl_vert_shader.to_string()),
                stage: ShaderStage::Vertex,
                defines: Default::default(),
            },
        });
        let frag_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Naga(Cow::Owned(naga_module)),
        });

        let bg_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let screen_ratio_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[window_size.width as f32, window_size.height as f32]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bg = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bg_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(screen_ratio_buffer.as_entire_buffer_binding()),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_shader,
                entry_point: "main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let config = surface.get_default_config(&adapter, window_size.width, window_size.height).unwrap();
        surface.configure(&device, &config);

        window.request_redraw();
        self.window = Some(window);

        self.handle_redraw = Box::new(move |event_loop, id, event| {
            let frame = surface.get_current_texture().expect("Failed to acquire next swap chain texture");
            let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rpass.set_pipeline(&render_pipeline);
                rpass.set_bind_group(0, &bg, &[]);

                // Render a single triangle takes up the entire screen.
                // Actual round objects will be rendered in the fragment shader.
                rpass.draw(0..3, 0..1);
            }

            queue.submit(Some(encoder.finish()));
            frame.present();
        });
    }
}
