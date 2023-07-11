use bevy::prelude::*;

mod camera_plugin;
mod config_plugin;
mod plane_plugin;

use camera_plugin::CameraPlugin;
use config_plugin::ConfigPlugin;
use plane_plugin::PlanePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(ConfigPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlanePlugin);
    }
}

use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    block_on(async {
        App::new().add_plugin(GamePlugin).run();
    });
}
