use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_tilemap::prelude::*;
use noise::{BasicMulti, NoiseFn, Perlin};

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let noise = BasicMulti::<Perlin>::new(72);

    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 28. * 16. * 4.,
                height: 28. * 16. * 4.,
            },
            ..OrthographicProjection::default_2d()
        },
        // OrthographicProjection {
        //     scaling_mode: ScalingMode::Fixed {
        //         width: 28. * 16. * 4. * 2.,
        //         height: 28. * 16. * 4. * 2.,
        //     },
        //     ..OrthographicProjection::default_2d()
        // },
        Transform::from_xyz(
            27. * 16. * 4. / 2.,
            27. * 16. * 4. / 2.,
            0.,
        ),
    ));

    let texture_handle: Handle<Image> =
        asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 28, y: 28 };

    // Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    commands.entity(tilemap_entity).with_children(
        |parent| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let should_render = noise.get([
                        x as f64 * 101.2,
                        y as f64 * 101.2,
                    ]) > 0.;
                    if should_render {
                        let tile_pos = TilePos { x, y };
                        let tile_entity = parent
                            .spawn(TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(
                                    tilemap_entity,
                                ),
                                texture_index:
                                    TileTextureIndex(
                                        (dbg!(
                                            (noise.get([
                                                tile_pos.x
                                                    as f64
                                                    * 22.2,
                                                tile_pos.y
                                                    as f64
                                                    * 22.2,
                                            ]) + 1. / 2.)
                                                * 3.
                                        )
                                            as u32),
                                    ),
                                color: TileColor(
                                    Color::WHITE
                                        .with_alpha(0.6),
                                ),
                                ..default()
                            })
                            .id();
                        tile_storage
                            .set(&tile_pos, tile_entity);
                    }
                }
            }
        },
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    // dbg!(get_tilemap_center_transform(
    //     &map_size, &grid_size, &map_type, 1.0,
    // ));
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(
            texture_handle.clone(),
        ),
        tile_size,
        // transform: get_tilemap_center_transform(
        //     &map_size, &grid_size, &map_type, 0.0,
        // )
        transform: Transform::default()
            .with_scale(Vec3::new(4., 4., 1.)),
        ..default()
    });
}

fn startup2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let noise = BasicMulti::<Perlin>::new(72);
    let texture_handle: Handle<Image> =
        asset_server.load("tiles.png");

    // Layer 2
    let map_size = TilemapSize {
        x: 28 * 4,
        y: 28 * 4,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).with_children(
        |parent| {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let should_render_top = noise.get([
                        (x / 4) as f64 * 101.2,
                        (y / 4) as f64 * 101.2,
                    ]) > 0.;
                    let should_render = noise.get([
                        x as f64 * 101.2,
                        y as f64 * 101.2,
                    ]) > -0.;

                    if should_render_top && should_render {
                        let tile_pos = TilePos { x, y };
                        let tile_entity = parent
                            .spawn(TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(
                                    tilemap_entity,
                                ),
                                texture_index:
                                    TileTextureIndex(2),
                                color: TileColor(
                                    Color::WHITE
                                        .with_alpha(0.6),
                                ),
                                ..default()
                            })
                            .id();
                        tile_storage
                            .set(&tile_pos, tile_entity);
                    }
                }
            }
        },
    );

    // dbg!(get_tilemap_center_transform(
    //     &map_size, &grid_size, &map_type, 1.0,
    // ));
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
        // transform:
        // transform: get_tilemap_center_transform(
        //     &map_size, &grid_size, &map_type, 1.0,
        // ),
        transform: Transform::from_xyz(
            -(tile_size.x * 4. / 2. - tile_size.x / 2.),
            -(tile_size.x * 4. / 2. - tile_size.x / 2.),
            1.,
        ),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Srgba::hex("0c4a6e").unwrap().into(),
        ))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Layers"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (startup, startup2))
        .run();
}
