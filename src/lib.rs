mod mine;

use bevy::prelude::*;
use mine::Mine;
use wasm_bindgen::prelude::*;

fn hello_world() {
    println!("hello world!");
}
struct Board;
//struct Board {
//grid: Vec<Vec<Mine>>,
//}

fn generate_board(mut commands: Commands) {
    commands.spawn().insert(Board);
}
struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(generate_board.system());
    }
}

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();
    app.add_plugin(BoardPlugin);
    app.add_system(hello_world.system());

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // TODO: add all your other stuff to `app` as usual

    app.run();
}
