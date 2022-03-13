use bevy::prelude::*;

mod in_game_mouse_handlers;
use in_game_mouse_handlers::InGameMousePlugin;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InGameMousePlugin);
    }
}
