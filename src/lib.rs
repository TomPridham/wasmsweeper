extern crate web_sys;
mod board;
mod cell;

use bevy::prelude::*;
use board::{generate_board, Board, BoardPlugin};
use cell::{BasicCell, Cell, NewCell};
use wasm_bindgen::prelude::*;

// A macro to provide `log!(..)`-style syntax for `console.log` logging.
#[macro_use]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // Add walls
    let wall_material = materials.add(Color::PINK.into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(400.0, 400.0);

    // left
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    });

    // bottom
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        material: wall_material,
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    });
}

// This system prints messages when you press or release the left mouse button:
fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut board: Query<(&Board)>,
    mut query: Query<(&BasicCell, &mut Transform, &mut Handle<ColorMaterial>)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        let window = windows.get_primary_mut().unwrap();
        if let Some(cursor) = window.cursor_position() {
            log!("{:?}", cursor);
            let cursor = cursor - Vec2::new(window.width(), window.height()) / 2.0;
            for (basic_cell, transform, mut mat_handle) in query.iter_mut() {
                if basic_cell.contains(cursor) {
                    *mat_handle = materials.add(Color::BLUE.into());

                    log!("hurray");
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(BoardPlugin);
    app.add_system(mouse_click_system.system());
    app.insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)));

    app.add_startup_system(setup.system());

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // TODO: add all your other stuff to `app` as usual

    app.run();
}
