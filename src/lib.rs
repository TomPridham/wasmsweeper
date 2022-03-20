extern crate web_sys;
mod components;
mod log;
mod mouse;

use components::ComponentsPlugin;
use mouse::MousePlugin;

use bevy::prelude::*;
use bevy_ui_navigation::systems::{
    default_gamepad_input, default_keyboard_input, default_mouse_input, InputMapping,
};
use bevy_ui_navigation::{FocusState, Focusable, NavEvent, NavigationPlugin};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

#[cfg(not(target_arch = "wasm32"))]
fn load_images() {}
// preload all the images on wasm since they are http requests and can take some time
#[cfg(target_arch = "wasm32")]
#[allow(unused_must_use)]
fn load_images(asset_server: Res<AssetServer>) {
    asset_server.load_untyped("fonts/FiraSans-Bold.ttf");
    asset_server.load_untyped("one.png");
    asset_server.load_untyped("flag.png");
    asset_server.load_untyped("two.png");
    asset_server.load_untyped("three.png");
    asset_server.load_untyped("four.png");
    asset_server.load_untyped("five.png");
    asset_server.load_untyped("six.png");
    asset_server.load_untyped("seven.png");
    asset_server.load_untyped("eight.png");
    asset_server.load_untyped("mine.png");
    asset_server.load_untyped("u_lose.png");
    asset_server.load_untyped("u_win.png");
}

fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

#[wasm_bindgen]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // menu navigation
        .add_plugin(NavigationPlugin)
        .add_system(default_keyboard_input)
        .add_system(default_gamepad_input)
        .add_system(default_mouse_input)
        .init_resource::<InputMapping>()
        // components
        .add_plugin(MousePlugin)
        .add_plugin(ComponentsPlugin)
        .add_state(AppState::MainMenu)
        //setup
        .add_startup_system(load_images)
        .add_startup_system(setup)
        .run();
}
