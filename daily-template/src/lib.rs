use bevy::{
    core_pipeline::{
        bloom::BloomSettings, tonemapping::Tonemapping,
    },
    prelude::*,
};
pub mod brick;
pub mod colors;
// pub mod materials;
pub mod menu;

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}
