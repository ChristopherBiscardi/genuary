use bevy::{prelude::*, window::WindowMode};
use bevy_mod_picking::{
    debug::DebugPickingMode, DefaultPickingPlugins,
};
use bevy_tweening::TweeningPlugin;
use bevy_xpbd_3d::prelude::*;
use candy_blocks::{
    brick::materials::{
        highlight_colliding_cubes, BrickMaterialPlugin,
        CustomMaterial,
    },
    camera_controller::CameraControllerPlugin,
    level::{self, setup_game, GRID_AOC_TEST},
    menu::MenuPlugin,
    setup, AppState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::hex("1e1e2e").unwrap(),
        ))
        .add_state::<AppState>()
        .add_plugins((
            MaterialPlugin::<CustomMaterial>::default(),
            // PhysicsPlugins::default(),
            // CameraControllerPlugin,
            // TweeningPlugin,
            // DefaultPickingPlugins,
            // PhysicsDebugPlugin::default(),
            MenuPlugin,
            BrickMaterialPlugin,
            CameraControllerPlugin,
        ))
        // .insert_resource(State::new(
        //     DebugPickingMode::Disabled,
        // ))
        .add_systems(Startup, setup)
        // .add_systems(
        //     Update,
        //     highlight_colliding_cubes
        //         .run_if(in_state(AppState::InGame)),
        // )
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .run();
}
