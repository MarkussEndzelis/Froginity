use wgpu;
use winit::window::Window;
use crate::game::Game;
use crate::ui::UI;
use crate::sprite::Sprite;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use crate::sprites::{FROG_PALETTE, FROG_SPRITE, GROUND_PALETTE, GROUND_SPRITE, OBSTACLE_PALETTE, OBSTACLE_SPRITE, DIGIT_SPRITES, DIGIT_PALETTE, LETTER_PALETTE, letter_sprite, BIRD_PALETTE, BIRD_SPRITE};
use crate::state::{VIRTUAL_WIDTH, VIRTUAL_HEIGHT};

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pipeline: wgpu::RenderPipeline,
    white_bind_group: wgpu::BindGroup,
    sky_texture_view: Arc<wgpu::TextureView>,
    digit_texture_views: Vec<Arc<wgpu::TextureView>>,
    letter_texture_views:std::collections::HashMap<char, Arc<wgpu::TextureView>>,
    pub white_texture_view: Arc<wgpu::TextureView>,
    pub frog_texture_view: Arc<wgpu::TextureView>,
    pub ground_texture_view: Arc<wgpu::TextureView>,
    pub obstacle_texture_view: Arc<wgpu::TextureView>,
    pub bird_texture_view: Arc<wgpu::TextureView>,
}

impl Renderer {
    pub async fn new(window: &Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor{
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shader.wgsl").into()),
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute  {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        };

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[vertex_layout],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            let white_rgba = vec![255u8, 255, 255, 255];
            let white_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("white_texture"),
                size: wgpu::Extent3d {width: 1, height: 1, depth_or_array_layers: 1},
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &white_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &white_rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d {width: 1, height: 1, depth_or_array_layers: 1},
            );
            let white_view = white_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let white_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });
            let white_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&white_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&white_sampler),
                    },
                ],
                label: Some("white_bind_group"),
            });

            let white_view = Arc::new(white_view);

            let frog_texture_view = Arc::new(Self::sprite_to_texture_view::<32, 32>(&device, &queue, &FROG_SPRITE, &FROG_PALETTE));
            let ground_texture_view = Arc::new(Self::sprite_to_texture_view::<16, 16>(&device, &queue, &GROUND_SPRITE, &GROUND_PALETTE));
            let obstacle_texture_view = Arc::new(Self::sprite_to_texture_view::<16, 16>(&device, &queue, &OBSTACLE_SPRITE, &OBSTACLE_PALETTE));
            let bird_texture_view = Arc::new(Self::sprite_to_texture_view::<16, 16>(&device, &queue, &BIRD_SPRITE, &BIRD_PALETTE));
            let digit_texture_views: Vec<Arc<wgpu::TextureView>> = DIGIT_SPRITES.iter().map(|d| Arc::new(Self::sprite_to_texture_view(&device, &queue, d, &DIGIT_PALETTE))).collect();

            let digit_texture_views: Vec<Arc<wgpu::TextureView>> = DIGIT_SPRITES.iter().map(|d| Arc::new(Self::sprite_to_texture_view(&device, &queue, d, &DIGIT_PALETTE))).collect();
            let mut letter_texture_views: std::collections::HashMap<char, Arc<wgpu::TextureView>> = std::collections::HashMap::new();
            for c in "AEGMOPRSV".chars(){
                if let Some(bitmap) = letter_sprite(c){
                    letter_texture_views.insert(c, Arc::new(Self::sprite_to_texture_view(&device, &queue, &bitmap, &LETTER_PALETTE)));
                }
            }

            let sky_rgba = vec![
                60, 140, 220, 255,
                180, 210, 255, 255,
            ];

            let sky_texture = device.create_texture(&wgpu::TextureDescriptor{
                label: Some("sky_texture"),
                size: wgpu::Extent3d {width: 1, height: 2, depth_or_array_layers: 1},
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            
            queue.write_texture(
                wgpu::ImageCopyTexture{
                    texture: &sky_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &sky_rgba,
                wgpu::ImageDataLayout{
                    offset: 0,
                    bytes_per_row: Some(4 * 1),
                    rows_per_image: Some(2),
                },
                wgpu::Extent3d{width: 1, height: 2, depth_or_array_layers: 1},
            );

            let sky_texture_view = Arc::new(sky_texture.create_view(&wgpu::TextureViewDescriptor::default()));

            Self {
                surface,
                device,
                queue,
                config,
                size,
                pipeline,
                white_bind_group,
                white_texture_view: white_view.clone(),
                frog_texture_view,
                ground_texture_view,
                obstacle_texture_view,
                bird_texture_view,
                sky_texture_view,
                digit_texture_views,
                letter_texture_views,
            }
        }



        pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
            if new_size.width > 0 && new_size.height > 0 {
                self.size = new_size;
                self.config.width = new_size.width;
                self.config.height = new_size.height;
                self.surface.configure(&self.device, &self.config);
            }
        }

        pub fn render(&mut self, game: &Game, ui: &UI) -> Result<(), wgpu::SurfaceError>{
            let output = self.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            let sw = self.size.width as f32;
            let sh = self.size.height as f32;

            let mut sprites_data = Vec::new();

            let sky_sprite = Sprite::new(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, [0.0,0.0,1.0,1.0], [1.0;4], self.sky_texture_view.clone());
            
            let (verts, inds) = sky_sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
            let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&inds),
                usage: wgpu::BufferUsages::INDEX,
            });
            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry{
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&sky_sprite.texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor {
                            mag_filter: wgpu::FilterMode::Linear,
                            min_filter: wgpu::FilterMode::Linear,
                            ..Default::default()
                        })),
                    },
                ],
                label: Some("bind_group"),
            });
            sprites_data.push((vb, ib, bind_group, inds.len()));

            for g in &game.grounds {
                let sprite = Sprite::new(
                    g.x, g.y, g.width, g.height,
                    [0.0, 0.0, 1.0, 1.0],
                    [1.0, 1.0, 1.0, 1.0],
                    self.ground_texture_view.clone(),
                );
                let (verts, inds) = sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&inds),
                    usage: wgpu::BufferUsages::INDEX,
                });
                let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry{
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&sprite.texture_view),
                        },
                        wgpu::BindGroupEntry{
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor{
                                mag_filter: wgpu::FilterMode::Nearest,
                                min_filter: wgpu::FilterMode::Nearest,
                                ..Default::default()
                            })),
                        },
                    ],
                    label: Some("bind_group"),
                });
                sprites_data.push((vb, ib, bind_group, inds.len()));
            }

            for obs in &game.obstacles {
                if obs.active {
                    let sprite = Sprite::new(
                        obs.x, obs.y, obs.width, obs.height,
                        [0.0, 0.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        self.obstacle_texture_view.clone(),
                    );
                    let (verts, inds) = sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                    let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&verts),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                    let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&inds),
                        usage: wgpu::BufferUsages::INDEX,
                    });
                    let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor{
                        layout: &self.pipeline.get_bind_group_layout(0),
                        entries: &[
                            wgpu::BindGroupEntry{
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(&sprite.texture_view),
                            },
                            wgpu::BindGroupEntry{
                                binding: 1,
                                resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor {
                                    mag_filter: wgpu::FilterMode::Nearest,
                                    min_filter: wgpu::FilterMode::Nearest,
                                    ..Default::default()
                                })),
                            },
                        ],
                        label: Some("bind_group"),
                    });
                    sprites_data.push((vb, ib, bind_group, inds.len()));
                }
            }

            for b in &game.birds {
                if b.active {
                    let sprite = Sprite::new(
                        b.x, b.y, b.width, b.height,
                        [0.0, 0.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        self.bird_texture_view.clone(),
                    );
                    let (verts, inds) = sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                    let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                        label: Some("Vertex Buffer"), contents: bytemuck::cast_slice(&verts), usage: wgpu::BufferUsages::VERTEX,
                    });
                    let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                        label: Some("Index Buffer"), contents: bytemuck::cast_slice(&inds), usage: wgpu::BufferUsages::INDEX,
                    });
                    let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor{
                        layout: &self.pipeline.get_bind_group_layout(0),
                        entries: &[
                            wgpu::BindGroupEntry{binding: 0, resource: wgpu::BindingResource::TextureView(&sprite.texture_view)},
                            wgpu::BindGroupEntry{binding: 1, resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor{
                                mag_filter: wgpu::FilterMode::Nearest, min_filter: wgpu::FilterMode::Nearest, ..Default::default()
                            }))},
                        ],
                        label: Some("bind_group"),
                    });
                    sprites_data.push((vb, ib, bind_group, inds.len()));
                }
            }

            {
                let frog_sprite = Sprite::new(
                    game.frog.x, game.frog.y,
                    game.frog.width, game.frog.height,
                    [1.0, 0.0, 0.0, 1.0],
                    [1.0, 1.0, 1.0, 1.0],
                    self.frog_texture_view.clone(),
                );
                let (verts, inds) = frog_sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&inds),
                    usage: wgpu::BufferUsages::INDEX,
                });
                let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor{
                    layout: &self.pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry{
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&frog_sprite.texture_view),
                        },
                        wgpu::BindGroupEntry{
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor {
                                mag_filter: wgpu::FilterMode::Nearest,
                                min_filter: wgpu::FilterMode::Nearest,
                                ..Default::default()
                            })),
                        },
                    ],
                    label: Some("bind_group"),
                });
                sprites_data.push((vb, ib, bind_group, inds.len()));
            }

            for item in ui.sprites(){
                let sprite = Sprite::new(
                    item.x, item.y, item.width, item.height,
                    [0.0, 0.0, 1.0, 1.0],
                    item.color,
                    self.white_texture_view.clone(),
                );
                let (verts, inds) = sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&verts),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&inds),
                    usage: wgpu::BufferUsages::INDEX,
                });
                let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor{
                    layout: &self.pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry{
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&sprite.texture_view),
                        },
                        wgpu::BindGroupEntry{
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor{
                                mag_filter: wgpu::FilterMode::Nearest,
                                min_filter: wgpu::FilterMode::Nearest,
                                ..Default::default()
                            })),
                        },
                    ],
                    label: Some("bind_group"),
                });
                sprites_data.push((vb, ib, bind_group, inds.len()));
            }

            let digit_w = 18.0;
            let digit_h = 30.0;
            let mut dx = 16.0;
            for &d in &ui.score_digits {
                let sprite = Sprite::new(dx, 10.0, digit_w, digit_h, [0.0,0.0,1.0,1.0], [1.0;4], self.digit_texture_views[d as usize].clone());
                let (verts, inds) = sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{ label: Some("Vertex Buffer"), contents: bytemuck::cast_slice(&verts), usage: wgpu::BufferUsages::VERTEX});
                let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{ label: Some("Index Buffer"), contents: bytemuck::cast_slice(&inds), usage: wgpu::BufferUsages::INDEX});
                let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor{
                    layout: &self.pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry{binding: 0, resource: wgpu::BindingResource::TextureView(&sprite.texture_view)},
                        wgpu::BindGroupEntry{binding: 1, resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor{mag_filter: wgpu::FilterMode::Nearest, min_filter: wgpu::FilterMode::Nearest, ..Default::default()}))},
                    ],
                    label: Some("bind_group"),
                });
                sprites_data.push((vb, ib, bind_group, inds.len()));
                dx += digit_w + 4.0;
            }

            if game.game_over {
                let draw_text = |text: &str, start_x: f32, y: f32, char_w: f32, char_h: f32, spacing: f32, color: [f32; 4], sprites_data: &mut Vec<_>| {
                    let mut dx = start_x;
                    for c in text.chars(){
                        if c == ' ' {dx += char_w + spacing; continue; }
                        if let Some(tex) = self.letter_texture_views.get(&c){
                            let sprite = Sprite::new(dx, y, char_w, char_h, [0.0,0.0,1.0,1.0], color, tex.clone());
                            let (verts, inds) = sprite.rect_to_vertices(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                            let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label: Some("Vertex Buffer"), contents: bytemuck::cast_slice(&verts), usage: wgpu::BufferUsages::VERTEX});
                            let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label: Some("Index Buffer"), contents: bytemuck::cast_slice(&inds), usage: wgpu::BufferUsages::INDEX});
                            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor{
                                layout: &self.pipeline.get_bind_group_layout(0),
                                entries: &[
                                    wgpu::BindGroupEntry{binding: 0, resource: wgpu::BindingResource::TextureView(&sprite.texture_view)},
                                    wgpu::BindGroupEntry{binding: 1, resource: wgpu::BindingResource::Sampler(&self.device.create_sampler(&wgpu::SamplerDescriptor{mag_filter: wgpu::FilterMode::Nearest, min_filter: wgpu::FilterMode::Nearest, ..Default::default()}))},
                                ],
                                label: Some("bind_group"),
                            });
                            sprites_data.push((vb, ib, bind_group, inds.len()));
                        }
                        dx += char_w + spacing;
                    }
                };
                draw_text("GAME_OVER", 303.0, 268.0, 18.0, 24.0, 4.0, [1.0, 1.0, 1.0, 1.0], &mut sprites_data);
                draw_text("PRESS R", 342.0, 328.0, 14.0, 18.0, 3.0, [0.1, 0.1, 0.15, 1.0], &mut sprites_data);
            }

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.3,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            for (vb, ib, bind_group, index_count) in &sprites_data {
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.set_vertex_buffer(0, vb.slice(..));
                render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..*index_count as u32, 0, 0..1);
            }

            drop(render_pass);

            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();

            Ok(())
        }

        pub fn create_texture_bind_group(&self, texture_view: &wgpu::TextureView) -> wgpu::BindGroup {
            let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
                label: Some("texture_bind_group"),
            })
        }

        pub fn white_bind_group(&self) -> &wgpu::BindGroup {
            &self.white_bind_group
        }

        fn sprite_to_texture_view<const W: usize, const H: usize>(
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            sprite: &[[u8; W]; H],
            palette: &[[u8; 4]],
        ) -> wgpu::TextureView {
            let width = W as u32;
            let height = H as u32;
            let mut data = Vec::with_capacity((width * height * 4) as usize);
            for row in sprite.iter(){
                for &px in row.iter(){
                    data.extend_from_slice(&palette[px as usize]);
                }
            }

            let texture_size = wgpu::Extent3d {width, height, depth_or_array_layers: 1};
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("sprite_texture"),
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            queue.write_texture(
                wgpu::ImageCopyTexture{
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &data,
                wgpu::ImageDataLayout{
                    offset: 0,
                    bytes_per_row: Some(4 * width),
                    rows_per_image: Some(height),
                },
                texture_size,
            );
            texture.create_view(&wgpu::TextureViewDescriptor::default())
        }
    }
