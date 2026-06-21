// Optional GPU rendering path using wgpu compute shaders.
//
// Enabled by the `gpu` Cargo feature. At startup, the global GPU_RENDERER is
// initialised once (lazily). If initialisation fails (no GPU, no Metal/Vulkan
// driver) the renderer is None and every `render_frame` call returns None,
// which triggers the existing CPU path in render.rs.
//
// Scenes supported by the GPU path (pure per-pixel math):
//   plasma, noise_clouds, tunnel, sine_wave, mandelbrot, sdf_shapes, gradient
//
// All other scenes fall through to the CPU path automatically.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::config::SceneConfig;
use crate::RenderContext;

// ── uniform buffer layout (64 bytes, matches WGSL FrameParams) ──────────────

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct GpuFrameParams {
    width:        u32,
    height:       u32,
    frame_index:  u32,
    total_frames: u32,
    norm_time:    f32,
    seed:         u32,
    _pad0:        u32,
    _pad1:        u32,
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
}

const _: () = assert!(std::mem::size_of::<GpuFrameParams>() == 64);

// ── GPU renderer ─────────────────────────────────────────────────────────────

pub struct GpuRenderer {
    device:            wgpu::Device,
    queue:             wgpu::Queue,
    bind_group_layout: wgpu::BindGroupLayout,
    pipelines:         HashMap<&'static str, wgpu::ComputePipeline>,
}

// Shader WGSL source: each scene file is prefixed with the common header.
const COMMON: &str = include_str!("shaders/common.wgsl");

fn scene_shader(name: &str) -> Option<&'static str> {
    match name {
        "plasma"       => Some(include_str!("shaders/plasma.wgsl")),
        "noise_clouds" => Some(include_str!("shaders/noise_clouds.wgsl")),
        "tunnel"       => Some(include_str!("shaders/tunnel.wgsl")),
        "sine_wave"    => Some(include_str!("shaders/sine_wave.wgsl")),
        "mandelbrot"   => Some(include_str!("shaders/mandelbrot.wgsl")),
        "sdf_shapes"   => Some(include_str!("shaders/sdf_shapes.wgsl")),
        "gradient"     => Some(include_str!("shaders/gradient.wgsl")),
        _ => None,
    }
}

const GPU_SCENES: &[&str] = &[
    "plasma", "noise_clouds", "tunnel", "sine_wave",
    "mandelbrot", "sdf_shapes", "gradient",
];

impl GpuRenderer {
    pub fn try_init() -> Option<Self> {
        pollster::block_on(Self::try_init_async())
    }

    async fn try_init_async() -> Option<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("frameseed"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    ..Default::default()
                },
                None,
            )
            .await
            .ok()?;

        // One bind-group layout shared across all compute pipelines:
        //   binding 0 — uniform buffer (GpuFrameParams)
        //   binding 1 — storage buffer (output RGBA pixels)
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("frameseed_bgl"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // pipeline_layout is referenced internally by each pipeline; drop it after.
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("frameseed_pl"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        // Compile a pipeline for every GPU-capable scene at startup.
        let mut pipelines = HashMap::new();
        for &scene_name in GPU_SCENES {
            let Some(scene_src) = scene_shader(scene_name) else {
                continue;
            };
            let full_src = format!("{COMMON}\n{scene_src}");
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(scene_name),
                source: wgpu::ShaderSource::Wgsl(full_src.into()),
            });
            let pipeline =
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(scene_name),
                    layout: Some(&pipeline_layout),
                    module: &shader,
                    entry_point: "main",
                    compilation_options: Default::default(),
                    cache: None,
                });
            pipelines.insert(scene_name, pipeline);
        }
        drop(pipeline_layout);

        Some(GpuRenderer {
            device,
            queue,
            bind_group_layout,
            pipelines,
        })
    }

    /// Render one frame on the GPU and return raw RGBA bytes, or `None` if the
    /// scene has no GPU shader or if any wgpu call fails.
    pub fn render_frame(
        &self,
        scene:  &SceneConfig,
        ctx:    &RenderContext,
        width:  u32,
        height: u32,
    ) -> Option<Vec<u8>> {
        let pipeline = self.pipelines.get(scene.name.as_str())?;
        let params   = build_params(scene, ctx, width, height);

        // ── buffers ─────────────────────────────────────────────────────────
        let uniform_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    None,
            contents: bytemuck::bytes_of(&params),
            usage:    wgpu::BufferUsages::UNIFORM,
        });

        let output_size = (width * height * 4) as wgpu::BufferAddress;
        let output_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label:              None,
            size:               output_size,
            usage:              wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let staging_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label:              None,
            size:               output_size,
            usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ── bind group ───────────────────────────────────────────────────────
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:  None,
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding:  0,
                    resource: uniform_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding:  1,
                    resource: output_buf.as_entire_binding(),
                },
            ],
        });

        // ── encode & dispatch ────────────────────────────────────────────────
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label:             None,
                    timestamp_writes:  None,
                });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((width + 7) / 8, (height + 7) / 8, 1);
        }
        encoder.copy_buffer_to_buffer(&output_buf, 0, &staging_buf, 0, output_size);
        self.queue.submit(std::iter::once(encoder.finish()));

        // ── readback ─────────────────────────────────────────────────────────
        let slice = staging_buf.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).ok();
        });
        self.device.poll(wgpu::Maintain::Wait);
        rx.recv().ok()?.ok()?;

        let data = {
            let view = slice.get_mapped_range();
            view.to_vec()
        };
        staging_buf.unmap();
        Some(data)
    }
}

// ── param builder ─────────────────────────────────────────────────────────────

fn build_params(scene: &SceneConfig, ctx: &RenderContext, width: u32, height: u32) -> GpuFrameParams {
    let (p0, p1, p2, p3, p4, p5, p6, p7) = scene_params(scene);
    GpuFrameParams {
        width,
        height,
        frame_index:  ctx.frame_index,
        total_frames: ctx.total_frames,
        norm_time:    ctx.normalized_time,
        seed:         (ctx.seed & 0xFFFF_FFFF) as u32,
        _pad0:        0,
        _pad1:        0,
        p0, p1, p2, p3, p4, p5, p6, p7,
    }
}

fn scene_params(scene: &SceneConfig) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    match scene.name.as_str() {
        "plasma" => (scene.plasma.speed, scene.plasma.scale, 0., 0., 0., 0., 0., 0.),
        "noise_clouds" => (
            scene.noise_clouds.speed,
            scene.noise_clouds.scale,
            if scene.noise_clouds.colored { 1.0 } else { 0.0 },
            0., 0., 0., 0., 0.,
        ),
        "tunnel"    => (scene.tunnel.speed, scene.tunnel.rings as f32, 0., 0., 0., 0., 0., 0.),
        "sine_wave" => (scene.sine_wave.speed, 0., 0., 0., 0., 0., 0., 0.),
        "mandelbrot" => {
            let m = &scene.mandelbrot;
            (m.max_iter as f32, m.center_re, m.center_im, m.initial_view_width, m.zoom_speed, 0., 0., 0.)
        }
        "sdf_shapes" => {
            let s = &scene.sdf_shapes;
            (s.circle_radius, s.box_half_width, s.box_half_height, s.speed, 0., 0., 0., 0.)
        }
        "gradient" => {
            let g = &scene.gradient;
            let (palette_start, palette_end) = crate::palette_from_name(&g.palette);
            let start = g.start_color.as_deref()
                .and_then(crate::Rgba::from_hex)
                .unwrap_or(palette_start);
            let end = g.end_color.as_deref()
                .and_then(crate::Rgba::from_hex)
                .unwrap_or(palette_end);
            let dir = match &g.direction {
                crate::scenes::GradientDirection::Horizontal => 0.,
                crate::scenes::GradientDirection::Vertical   => 1.,
                crate::scenes::GradientDirection::Radial     => 2.,
                crate::scenes::GradientDirection::Diagonal   => 3.,
            };
            (g.speed, dir,
             start.r as f32 / 255.0, start.g as f32 / 255.0, start.b as f32 / 255.0,
             end.r as f32 / 255.0,   end.g as f32 / 255.0,   end.b as f32 / 255.0)
        }
        _ => (0., 0., 0., 0., 0., 0., 0., 0.),
    }
}

// ── global renderer (initialised once, reused across all frames) ─────────────

static GPU_RENDERER: OnceLock<Option<Mutex<GpuRenderer>>> = OnceLock::new();

/// Try to render one frame via GPU.  Returns `None` if GPU is unavailable or
/// the scene has no compute shader — caller should fall back to CPU.
pub fn try_gpu_render(
    scene:  &SceneConfig,
    ctx:    &RenderContext,
    width:  u32,
    height: u32,
) -> Option<Vec<u8>> {
    let renderer_opt = GPU_RENDERER.get_or_init(|| {
        GpuRenderer::try_init().map(Mutex::new)
    });
    let mutex = renderer_opt.as_ref()?;
    let renderer = mutex.lock().ok()?;
    renderer.render_frame(scene, ctx, width, height)
}

/// Render all frames sequentially on the GPU in scene order.  Returns None if
/// the GPU path is unavailable for this scene, so caller can use CPU parallel.
pub fn try_gpu_render_all(
    config: &crate::config::RenderConfig,
    on_progress: &(impl Fn(u32, u32) + Send + Sync),
) -> Option<Vec<Vec<u8>>> {
    // Quick check: does this scene have a GPU shader?
    scene_shader(config.scene.name.as_str())?;

    let total = config.total_frames();
    let mut out = Vec::with_capacity(total as usize);

    for i in 0..total {
        let ctx = RenderContext::new(i, total, config.fps, config.seed);
        let buf = try_gpu_render(&config.scene, &ctx, config.width, config.height)?;
        out.push(buf);
        on_progress(i + 1, total);
    }
    Some(out)
}
