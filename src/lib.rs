mod mine;

use bevy::prelude::*;
use mine::Mine;
use wasm_bindgen::prelude::*;

fn hello_world() {
    println!("hello world!");
}
struct Board {
    height: u16,
    width: u16,
}
//struct Board {
//grid: Vec<Vec<Mine>>,
//}

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
    let mine_spacing = 2.0;
    let mine_size = Vec2::new(96.0, 96.0);
    let mines_width = mine_columns as f32 * (mine_size.x + mine_spacing) - mine_spacing;
    // center the mines and move them up a bit
    let mines_offset = Vec3::new(
        -(mines_width - mine_size.x) / 2.0,
        -(mines_width - mine_size.y) / 2.0,
        0.0,
    );
    let mine_material = materials.add(Color::INDIGO.into());
    for row in 0..mine_rows {
        let y_position = row as f32 * (mine_size.y + mine_spacing);
        for column in 0..mine_columns {
            let mine_position = Vec3::new(
                column as f32 * (mine_size.x + mine_spacing),
                y_position,
                0.0,
            ) + mines_offset;
            // mine
            commands.spawn_bundle(SpriteBundle {
                material: mine_material.clone(),
                sprite: Sprite::new(mine_size),
                transform: Transform::from_translation(mine_position),
                ..Default::default()
            });
        }
    }
}
#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(BoardPlugin);
    app.add_system(hello_world.system());
    app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));

    app.add_startup_system(setup.system());

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // TODO: add all your other stuff to `app` as usual

    app.run();
}
