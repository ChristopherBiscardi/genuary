use bevy::prelude::*;
use bevy_app_compute::prelude::*;

pub struct GenuaryComputePlugin;

impl Plugin for GenuaryComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppComputeWorkerPlugin::<
            GenuaryComputeWorker,
        >::default(),))
            .add_systems(Update, genuary_compute_system);
    }
}

#[derive(TypeUuid)]
#[uuid = "2545ae14-a9bc-4f03-9ea4-4eb43d1075a7"]
struct GenuaryComputeShader;

impl ComputeShader for GenuaryComputeShader {
    fn shader() -> ShaderRef {
        "genuary_compute_shader.wgsl".into()
    }
}
#[derive(Resource)]
struct GenuaryComputeWorker;

impl ComputeWorker for GenuaryComputeWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let worker = AppComputeWorkerBuilder::new(world)
            // Add a uniform variable
            .add_uniform("uni", &5.)
            // Add a staging buffer, it will be available from
            // both CPU and GPU land.
            .add_staging("values", &[1., 2., 3., 4.])
            // Create a compute pass from your compute shader
            // and define used variables
            .add_pass::<GenuaryComputeShader>(
                [4, 1, 1],
                &["uni", "values"],
            )
            .build();

        worker
    }
}

fn genuary_compute_system(
    mut compute_worker: ResMut<
        AppComputeWorker<GenuaryComputeWorker>,
    >,
    mut local: Local<usize>,
) {
    if !compute_worker.ready() {
        return;
    };

    let result: Vec<f32> =
        compute_worker.read_vec("values");

    compute_worker.write_slice("values", &[2., 3.]);

    println!("got {:?}", result);
}
