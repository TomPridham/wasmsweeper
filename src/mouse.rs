use crate::board::Board;
use crate::cell::{BasicCell, CELL_COLOR};
use bevy::prelude::*;

pub fn left_click(
    mouse_button_input: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<(&BasicCell, &mut Handle<ColorMaterial>)>,
    asset_server: Res<AssetServer>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        let window = windows.get_primary_mut().unwrap();
        if let Some(cursor) = window.cursor_position() {
            let cursor = cursor - Vec2::new(window.width(), window.height()) / 2.0;
            for (basic_cell, mut mat_handle) in cell_query.iter_mut() {
                if basic_cell.contains(cursor) {
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
                        if cell.mine {
                            *mat_handle = materials.add(asset_server.load("mine.png").into());
                        } else {
                            match cell.value {
                                1 => {
                                    *mat_handle = materials.add(asset_server.load("one.png").into())
                                }
                                2 => {
                                    *mat_handle = materials.add(asset_server.load("two.png").into())
                                }
                                3 => {
                                    *mat_handle =
                                        materials.add(asset_server.load("three.png").into())
                                }
                                4 => {
                                    *mat_handle =
                                        materials.add(asset_server.load("four.png").into())
                                }
                                5 => {
                                    *mat_handle =
                                        materials.add(asset_server.load("five.png").into())
                                }
                                6 => {
                                    *mat_handle = materials.add(asset_server.load("six.png").into())
                                }
                                7 => {
                                    *mat_handle =
                                        materials.add(asset_server.load("seven.png").into())
                                }
                                8 => {
                                    *mat_handle =
                                        materials.add(asset_server.load("eight.png").into())
                                }
                                _ => *mat_handle = materials.add(Color::GRAY.into()),
                            }
                        }
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

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(left_click.system());
        app.add_system(right_click.system());
    }
}
