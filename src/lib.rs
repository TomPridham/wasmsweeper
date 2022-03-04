extern crate web_sys;
mod board;
mod cell;
mod log;
mod mouse;

use bevy::prelude::*;
use board::BoardPlugin;
use cell::CellPlugin;
use mouse::MousePlugin;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// preload all the images on wasm since they are http requests and can take some time
#[cfg(target_arch = "wasm32")]
#[allow(unused_must_use)]
fn load_images(asset_server: Res<AssetServer>) {
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
    // Add walls
    let wall_thickness = 10.0;
    let bounds = Vec2::new(400.0, 400.0);

    // left
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: Color::PINK,
            custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: Color::PINK,
            custom_size: Some(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });

    // bottom
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite {
            color: Color::PINK,
            custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite {
            color: Color::PINK,
            custom_size: Some(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(MousePlugin);
    app.add_plugin(BoardPlugin);
    app.add_plugin(CellPlugin);
    app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));

    #[cfg(target_arch = "wasm32")]
    app.add_startup_system(load_images);
    app.add_startup_system(setup);

    app.run();
}
