//! Demonstrates using a custom extension to the `StandardMaterial` to modify the results of the builtin pbr shader.

use std::f32::consts::PI;

use bevy::{
    pbr::{
        CascadeShadowConfigBuilder, ExtendedMaterial,
        MaterialExtension, OpaqueRendererMethod,
    },
    prelude::*,
    render::render_resource::*,
};
use gen_04_pixels::{colors, PixelatedExtension};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest()),))
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<
                StandardMaterial,
                PixelatedExtension,
            >,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<
            ExtendedMaterial<
                StandardMaterial,
                PixelatedExtension,
            >,
        >,
    >,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
) {
    // sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 5,
            })
            .unwrap(),
        ),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::RED,
                // can be used in forward or deferred mode.
                opaque_render_method:
                    OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                perceptual_roughness: 1.0,
                ..Default::default()
            },
            extension: PixelatedExtension {
                quantize_steps: 5,
            },
        }),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Plane {
                size: 100.,
                subdivisions: 1,
            })
            .unwrap(),
        ),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials_std.add(StandardMaterial {
            base_color: colors::GREEN,
            // can be used in forward or deferred mode.
            opaque_render_method:
                OpaqueRendererMethod::Auto,
            // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
            // in forward mode, the output can also be modified after lighting is applied.
            // see the fragment shader `extended_material.wgsl` for more info.
            // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
            // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
            perceptual_roughness: 1.0,
            ..Default::default()
        }),
        ..default()
    });

    // light
    commands.spawn((PointLightBundle::default(), Rotate));
    // directional 'sun' light
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 0.5,
    // });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }
        // .into(),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component)]
struct Rotate;

fn rotate_things(
    mut q: Query<&mut Transform, With<Rotate>>,
    time: Res<Time>,
) {
    for mut t in q.iter_mut() {
        t.translation = Vec3::new(
            time.elapsed_seconds().sin(),
            1.5,
            time.elapsed_seconds().cos(),
        ) * 4.0;
    }
}
