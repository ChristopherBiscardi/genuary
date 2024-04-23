//! A compute shader that simulates Conway's Game of Life.
//!
//! Compute shaders use the GPU for computing arbitrary information, that may be independent of what
//! is rendered to the screen.
use bevy::{
    core_pipeline::{
        bloom::BloomSettings, tonemapping::Tonemapping,
    },
    prelude::*,
    render::{
        extract_resource::{
            ExtractResource, ExtractResourcePlugin,
        },
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        Render, RenderApp, RenderSet,
    },
    window::WindowPlugin,
};
use gen_01_particles_2::colors;
use itertools::Itertools;
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Checkerboard, OpenSimplex, Simplex, Worley,
};
use std::{borrow::Cow, iter::repeat};

// const SIZE: (u32, u32) = (1280, 720);
// const SIZE: (u32, u32) = (1920, 1080);
const SIZE: (u32, u32) = (3840, 2160);
// const SIZE: (u32, u32) = (160, 90);
const WORKGROUP_SIZE: u32 = 8;

fn main() {
    // dbg!(
    //     colors::ROSEWATER.as_rgba_f32(),
    //     colors::FLAMINGO.as_rgba_f32(),
    //     colors::PINK.as_rgba_f32(),
    //     colors::MAUVE.as_rgba_f32(),
    //     colors::RED.as_rgba_f32(),
    //     colors::MAROON.as_rgba_f32(),
    //     colors::PEACH.as_rgba_f32(),
    //     colors::YELLOW.as_rgba_f32(),
    //     colors::GREEN.as_rgba_f32(),
    //     colors::TEAL.as_rgba_f32(),
    //     colors::SKY.as_rgba_f32(),
    //     colors::SAPPHIRE.as_rgba_f32(),
    //     colors::BLUE.as_rgba_f32(),
    //     colors::LAVENDER.as_rgba_f32()
    // );
    // panic!("");
    App::new()
        .insert_resource(ClearColor(colors::BASE))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // uncomment for unthrottled FPS
                    // present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            GameOfLifeComputePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // let noise_gen = Checkerboard::new(0);
    let noise_gen = OpenSimplex::new(0);

    // let noise_map =
    //     PlaneMapBuilder::<Checkerboard, 5>::new(noise_gen)
    //         .set_size(SIZE.0 as usize, SIZE.1 as usize)
    //         .set_x_bounds(-10.0, 10.0)
    //         .set_y_bounds(-10.0, 10.0)
    //         .build();
    // let noise_map =
    //     PlaneMapBuilder::<OpenSimplex, 3>::new(noise_gen)
    //         .set_size(SIZE.0 as usize, SIZE.1 as usize)
    //         .set_x_bounds(-100.0, 100.0)
    //         .set_y_bounds(-100.0, 100.0)
    //         .build();
    let noise_map =
        PlaneMapBuilder::<Worley, 3>::new(Worley::new(0))
            .set_size(SIZE.0 as usize, SIZE.1 as usize)
            // .set_x_bounds(-5.0, 5.0)
            // .set_y_bounds(-20.0, 20.0)
            .build();
    // &noise_map
    // .iter()
    // .chunks(4)
    // .into_iter()
    // .flat_map(|n| {
    //     let n = *n.into_iter().next().unwrap();
    //     // repeat((n * 256.) as u8).take(4)
    //     let color = if n < 1. / 13. {
    //         colors::ROSEWATER
    //     } else if n < 2. / 13. {
    //         colors::FLAMINGO
    //     } else if n < 3. / 13. {
    //         colors::PINK
    //     } else if n < 4. / 13. {
    //         colors::MAUVE
    //     } else if n < 5. / 13. {
    //         colors::RED
    //     } else if n < 6. / 13. {
    //         colors::MAROON
    //     } else if n < 7. / 13. {
    //         colors::PEACH
    //     } else if n < 8. / 13. {
    //         colors::YELLOW
    //     } else if n < 9. / 13. {
    //         colors::GREEN
    //     } else if n < 10. / 13. {
    //         colors::TEAL
    //     } else if n < 11. / 13. {
    //         colors::SKY
    //     } else if n < 12. / 13. {
    //         colors::SAPPHIRE
    //     } else if n < 13. / 13. {
    //         colors::BLUE
    //     } else {
    //         colors::LAVENDER
    //     };
    //     // return 4 u8s
    //     color.as_rgba_u8()
    // })
    // .collect::<Vec<u8>>(),
    let mut noise_image = Image::new_fill(
        Extent3d {
            width: noise_map.size().0 as u32,
            height: noise_map.size().1 as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        {
            let count = ((SIZE.1 / 14) * SIZE.0) as usize;
            &repeat(colors::ROSEWATER)
                .take(count)
                .chain(repeat(colors::FLAMINGO).take(count))
                .chain(repeat(colors::PINK).take(count))
                .chain(repeat(colors::MAUVE).take(count))
                .chain(repeat(colors::RED).take(count))
                .chain(repeat(colors::MAROON).take(count))
                .chain(repeat(colors::PEACH).take(count))
                .chain(repeat(colors::YELLOW).take(count))
                .chain(repeat(colors::GREEN).take(count))
                .chain(repeat(colors::TEAL).take(count))
                .chain(repeat(colors::SKY).take(count))
                .chain(repeat(colors::SAPPHIRE).take(count))
                .chain(repeat(colors::BLUE).take(count))
                .chain(repeat(colors::LAVENDER).take(count))
                .flat_map(|color| color.as_rgba_u8())
                .collect::<Vec<u8>>()
        },
        TextureFormat::Rgba8Unorm,
    );
    noise_image.texture_descriptor.usage =
        TextureUsages::COPY_DST
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;
    let noise_image = images.add(noise_image);
    commands.insert_resource(VectorField(noise_image));

    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage = TextureUsages::COPY_DST
        | TextureUsages::STORAGE_BINDING
        | TextureUsages::TEXTURE_BINDING;
    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(
                SIZE.0 as f32 * 0.5,
                SIZE.1 as f32 * 0.5,
            )),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    commands.insert_resource(GameOfLifeImage(image));
}

pub struct GameOfLifeComputePlugin;

impl Plugin for GameOfLifeComputePlugin {
    fn build(&self, app: &mut App) {
        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins((ExtractResourcePlugin::<
            GameOfLifeImage,
        >::default(),
        ExtractResourcePlugin::<
        VectorField,
    >::default()));
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_bind_group
                .in_set(RenderSet::PrepareBindGroups),
        );

        let mut render_graph =
            render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(
            "game_of_life",
            GameOfLifeNode::default(),
        );
        render_graph.add_node_edge(
            "game_of_life",
            bevy::render::main_graph::node::CAMERA_DRIVER,
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<GameOfLifePipeline>();
    }
}

#[derive(Resource, Clone, Deref, ExtractResource)]
struct GameOfLifeImage(Handle<Image>);

#[derive(Resource, Clone, Deref, ExtractResource)]
struct VectorField(Handle<Image>);

#[derive(Resource)]
struct GameOfLifeImageBindGroup(BindGroup);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<GameOfLifePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    game_of_life_image: Res<GameOfLifeImage>,
    vector_field_image: Res<VectorField>,
    render_device: Res<RenderDevice>,
) {
    let view =
        gpu_images.get(&game_of_life_image.0).unwrap();
    let view_noise =
        gpu_images.get(&vector_field_image.0).unwrap();

    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &view.texture_view,
            &view_noise.texture_view,
        )),
    );
    commands.insert_resource(GameOfLifeImageBindGroup(
        bind_group,
    ));
}

#[derive(Resource)]
pub struct GameOfLifePipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for GameOfLifePipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadOnly,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    }],
                });
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/game_of_life.wgsl");
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

        GameOfLifePipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum GameOfLifeState {
    Loading,
    Init,
    Update,
}

struct GameOfLifeNode {
    state: GameOfLifeState,
}

impl Default for GameOfLifeNode {
    fn default() -> Self {
        Self {
            state: GameOfLifeState::Loading,
        }
    }
}

impl render_graph::Node for GameOfLifeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline =
            world.resource::<GameOfLifePipeline>();
        let pipeline_cache =
            world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            GameOfLifeState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache
                        .get_compute_pipeline_state(
                            pipeline.init_pipeline,
                        )
                {
                    self.state = GameOfLifeState::Init;
                }
            }
            GameOfLifeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache
                        .get_compute_pipeline_state(
                            pipeline.update_pipeline,
                        )
                {
                    self.state = GameOfLifeState::Update;
                }
            }
            GameOfLifeState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group =
            &world.resource::<GameOfLifeImageBindGroup>().0;
        let pipeline_cache =
            world.resource::<PipelineCache>();
        let pipeline =
            world.resource::<GameOfLifePipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(
                &ComputePassDescriptor::default(),
            );

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
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
            GameOfLifeState::Update => {
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
