use crate::board::Board;
use crate::cell::{BasicCell, CELL_COLOR, SURROUND};
use bevy::prelude::*;

pub fn left_click(
    asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<(&BasicCell, &mut Handle<ColorMaterial>)>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut selected_cell: ResMut<SelectedCell>,
    mut windows: ResMut<Windows>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        let window = windows.get_primary_mut().unwrap();
        if let Some(cursor) = window.cursor_position() {
            let cursor = cursor - Vec2::new(window.width(), window.height()) / 2.0;
            for (basic_cell, mut mat_handle) in cell_query.iter_mut() {
                if basic_cell.contains(cursor) {
                    selected_cell.entity = Some((basic_cell.row, basic_cell.column));
                    if let Some(mut board) = board_query.iter_mut().next() {
                        let row = basic_cell.row;
                        let column = basic_cell.column;
                        if !board.initialized {
                            board.fill_board(4, (row, column)).unwrap();
                        }
                        let cell = &board.cells[row][column];
                        if cell.flagged {
                            break;
                        }
                        basic_cell.apply_material(
                            &asset_server,
                            materials,
                            &mut mat_handle,
                            cell.mine,
                            cell.value,
                        );
                    }
                    break;
                }
            }
        }
    }
}

pub fn right_click(
    mouse_button_input: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<(&BasicCell, &mut Handle<ColorMaterial>)>,
    asset_server: Res<AssetServer>,
) {
    if mouse_button_input.just_released(MouseButton::Right) {
        let window = windows.get_primary_mut().unwrap();
        if let Some(cursor) = window.cursor_position() {
            let cursor = cursor - Vec2::new(window.width(), window.height()) / 2.0;
            for (basic_cell, mut mat_handle) in cell_query.iter_mut() {
                if basic_cell.contains(cursor) {
                    if let Some(mut board) = board_query.iter_mut().next() {
                        let row = basic_cell.row;
                        let column = basic_cell.column;
                        let cell = &mut board.cells[row][column];
                        cell.flagged = !cell.flagged;
                        if cell.flagged {
                            *mat_handle = materials.add(asset_server.load("flag.png").into());
                        } else {
                            *mat_handle = materials.add(CELL_COLOR.into());
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SelectedCell {
    entity: Option<(usize, usize)>,
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedCell>();
        app.add_system(left_click.system().label("left_click"));
        app.add_system(right_click.system());
    }
}
