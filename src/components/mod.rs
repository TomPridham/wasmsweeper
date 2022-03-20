use bevy::prelude::*;

mod board;
mod cell;
mod menu;

pub use board::*;
pub use cell::*;
pub use menu::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BoardPlugin);
        app.add_plugin(CellPlugin);
        app.add_plugin(MenuPlugin);
    }
}
