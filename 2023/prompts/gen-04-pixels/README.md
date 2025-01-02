Must

```rust
fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(
                    colors::SKY,
                ),
                ..default()
            },
            camera: Camera {
                // hdr: not required, but fun!
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(0.0, 10.0, 15.0),
            )
            .looking_at(Vec3::new(0., 4., 0.), Vec3::Y),
            // I like TonyMcMapface
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            // Boosting the saturation can be interesting, but is not required
            color_grading: ColorGrading {
                post_saturation: 1.8,
                ..default()
            },
            projection: Projection::Orthographic(OrthographicProjection{
                scale: 0.1,
                ..default()
            }),
            ..default()
        },
        // depth prepass is required for pixelated.wgsl
        DepthPrepass,
        // normal prepass is required for pixelated.wgsl
        NormalPrepass,
        // PixelatedCamera causes this camera to be used to generate the
        // pixelated scene
        PixelatedCamera,
    ));
}
```

```rust
mut pixelated: ResMut<
    Assets<
        ExtendedMaterial<
            StandardMaterial,
            PixelatedExtension,
        >,
    >,
>,
```
