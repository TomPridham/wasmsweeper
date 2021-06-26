extern crate web_sys;
mod mine;

use bevy::{prelude::*, window::CursorMoved};
use mine::Mine;
use wasm_bindgen::prelude::*;

// A macro to provide `log!(..)`-style syntax for `console.log` logging.
#[macro_use]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

struct Board {
    height: u16,
    width: u16,
}

fn generate_board(mut commands: Commands) {
    commands.spawn().insert(Board {
        height: 16,
        width: 16,
    });
}

struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(generate_board.system());
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

    let mine_rows = 4;
    let mine_columns = 4;
    let spacing = 2.0;
    let size = Vec2::new(96.0, 96.0);
    let width = mine_columns as f32 * (size.x + spacing) - spacing;
    // center the mines and move them up a bit
    let offset = Vec3::new(-(width - size.x) / 2.0, -(width - size.y) / 2.0, 0.0);
    for row in 0..mine_rows {
        let y = row as f32 * (size.y + spacing);
        for column in 0..mine_columns {
            let mine_material = materials.add(Color::INDIGO.into());
            let mine_position = Vec3::new(column as f32 * (size.x + spacing), y, 0.0) + offset;
            // mine
            commands
                .spawn_bundle(SpriteBundle {
                    material: mine_material.clone(),
                    sprite: Sprite::new(size),
                    transform: Transform::from_translation(mine_position),
                    ..Default::default()
                })
                .insert(Mine {
                    mine: false,
                    value: 0,
                    x: column,
                    y: row,
                });
        }
    }
}

// This system prints messages when you press or release the left mouse button:
fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut windows: ResMut<Windows>,
    mut query: Query<(&Mine, &mut Transform)>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        let window = windows.get_primary_mut().unwrap();
        log!("left mouse just released, {:?}", window.cursor_position());
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
