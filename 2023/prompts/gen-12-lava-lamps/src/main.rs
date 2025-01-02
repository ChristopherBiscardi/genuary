//! A compute shader that simulates Conway's Game of Life.
//!
//! Compute shaders use the GPU for computing arbitrary information, that may be independent of what
//! is rendered to the screen.

use bevy::{
    prelude::*,
    render::{
        extract_resource::{
            ExtractResource, ExtractResourcePlugin,
        },
        render_asset::RenderAssetPersistencePolicy,
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::{
            binding_types::{
                texture_storage_2d, uniform_buffer,
            },
            *,
        },
        renderer::{
            RenderContext, RenderDevice, RenderQueue,
        },
        Render, RenderApp, RenderSet,
    },
    window::WindowPlugin,
};
use std::borrow::Cow;

const SIZE: (u32, u32) = (1280, 720);
const WORKGROUP_SIZE: u32 = 8;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // uncomment for unthrottled FPS
                    // present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            RayTracingComputePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetPersistencePolicy::Unload,
    );
    image.texture_descriptor.usage = TextureUsages::COPY_DST
        | TextureUsages::STORAGE_BINDING
        | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(
                SIZE.0 as f32,
                SIZE.1 as f32,
            )),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(RayTracingImage(image));
}

pub struct RayTracingComputePlugin;

impl Plugin for RayTracingComputePlugin {
    fn build(&self, app: &mut App) {
        // Extract the raytraced image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins((
            ExtractResourcePlugin::<
                RayTracingImage,
            >::default(),
            ExtractResourcePlugin::<
                ExtractedTime,
            >::default()
        ));
        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(
            Render,
            (
                prepare_bind_group
                    .in_set(RenderSet::PrepareBindGroups),
                prepare_time.in_set(RenderSet::Prepare),
            ),
        );

        let mut render_graph =
            render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(
            "ray_tracing",
            RayTracingNode::default(),
        );
        render_graph.add_node_edge(
            "ray_tracing",
            bevy::render::main_graph::node::CAMERA_DRIVER,
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<RayTracingPipeline>();
    }
}

#[derive(Resource, Clone, Deref, ExtractResource)]
struct RayTracingImage(Handle<Image>);

#[derive(Resource)]
struct RayTracingImageBindGroup(BindGroup);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<RayTracingPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    ray_tracing_image: Res<RayTracingImage>,
    render_device: Res<RenderDevice>,
    time_meta: ResMut<TimeMeta>,
) {
    let view =
        gpu_images.get(&ray_tracing_image.0).unwrap();
    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &view.texture_view,
            time_meta.buffer.as_entire_binding(),
        )),
    );
    commands.insert_resource(RayTracingImageBindGroup(
        bind_group,
    ));
}

#[derive(Resource)]
pub struct RayTracingPipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for RayTracingPipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout = world
            .resource::<RenderDevice>()
            .create_bind_group_layout(
                None,
                &BindGroupLayoutEntries::sequential(
                    ShaderStages::COMPUTE,
                    (
                        texture_storage_2d(
                            TextureFormat::Rgba8Unorm,
                            StorageTextureAccess::ReadWrite,
                        ),
                        uniform_buffer::<f32>(false),
                    ),
                ),
            );
        let render_device =
            world.resource::<RenderDevice>();
        let buffer = render_device.create_buffer(
            &BufferDescriptor {
                label: Some("time uniform buffer"),
                size: std::mem::size_of::<f32>() as u64,
                usage: BufferUsages::UNIFORM
                    | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            },
        );
        world.insert_resource(TimeMeta {
            buffer,
            bind_group: None,
        });

        let shader = world
            .resource::<AssetServer>()
            .load("lava_lamp.wgsl");
        let pipeline_cache =
            world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache
            .queue_compute_pipeline(
                ComputePipelineDescriptor {
                    label: None,
                    layout: vec![
                        texture_bind_group_layout.clone()
                    ],
                    push_constant_ranges: Vec::new(),
                    shader: shader.clone(),
                    shader_defs: vec![],
                    entry_point: Cow::from("init"),
                },
            );
        let update_pipeline = pipeline_cache
            .queue_compute_pipeline(
                ComputePipelineDescriptor {
                    label: None,
                    layout: vec![
                        texture_bind_group_layout.clone()
                    ],
                    push_constant_ranges: Vec::new(),
                    shader,
                    shader_defs: vec![],
                    entry_point: Cow::from("update"),
                },
            );

        RayTracingPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum RayTracingState {
    Loading,
    Init,
    Update,
}

struct RayTracingNode {
    state: RayTracingState,
}

impl Default for RayTracingNode {
    fn default() -> Self {
        Self {
            state: RayTracingState::Loading,
        }
    }
}

impl render_graph::Node for RayTracingNode {
    fn update(&mut self, world: &mut World) {
        let pipeline =
            world.resource::<RayTracingPipeline>();
        let pipeline_cache =
            world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            RayTracingState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache
                        .get_compute_pipeline_state(
                            pipeline.init_pipeline,
                        )
                {
                    self.state = RayTracingState::Init;
                }
            }
            RayTracingState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache
                        .get_compute_pipeline_state(
                            pipeline.update_pipeline,
                        )
                {
                    self.state = RayTracingState::Update;
                }
            }
            RayTracingState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group =
            &world.resource::<RayTracingImageBindGroup>().0;
        let pipeline_cache =
            world.resource::<PipelineCache>();
        let pipeline =
            world.resource::<RayTracingPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(
                &ComputePassDescriptor::default(),
            );

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            RayTracingState::Loading => {}
            RayTracingState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(
                        pipeline.init_pipeline,
                    )
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    SIZE.0 / WORKGROUP_SIZE,
                    SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
            RayTracingState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(
                        pipeline.update_pipeline,
                    )
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    SIZE.0 / WORKGROUP_SIZE,
                    SIZE.1 / WORKGROUP_SIZE,
                    1,
                );
            }
        }

        Ok(())
    }
}

#[derive(Resource, Default)]
struct ExtractedTime {
    seconds_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(time: &Self::Source) -> Self {
        ExtractedTime {
            seconds_since_startup: time.elapsed_seconds(),
        }
    }
}

#[derive(Resource)]
struct TimeMeta {
    buffer: Buffer,
    bind_group: Option<BindGroup>,
}

// write the extracted time into the corresponding uniform buffer
fn prepare_time(
    time: Res<ExtractedTime>,
    time_meta: ResMut<TimeMeta>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &time_meta.buffer,
        0,
        bevy::core::cast_slice(&[
            time.seconds_since_startup
        ]),
    );
}
