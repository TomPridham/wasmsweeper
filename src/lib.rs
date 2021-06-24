use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // TODO: add all your other stuff to `app` as usual

    app.run();
}
