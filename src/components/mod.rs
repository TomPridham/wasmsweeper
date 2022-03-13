use bevy::prelude::*;

mod board;
mod cell;

pub use board::*;
pub use cell::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BoardPlugin);
        app.add_plugin(CellPlugin);
    }
}
