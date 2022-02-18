use crate::board::{AllCellsOpenedEvent, Board, ClearOpenCellsEvent, MineClickedEvent};
use crate::cell::{ApplyMaterialEvent, BasicCell, CELL_COLOR};
use bevy::prelude::*;

pub fn left_click(
    mouse_button_input: Res<Input<MouseButton>>,
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<&BasicCell>,
    mut windows: ResMut<Windows>,
    mut ev_apply_material: EventWriter<ApplyMaterialEvent>,
    mut ev_open_cells: EventWriter<ClearOpenCellsEvent>,
    mut ev_mine_clicked: EventWriter<MineClickedEvent>,
    mut ev_all_opened: EventWriter<AllCellsOpenedEvent>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if let Some(board) = board_query.iter_mut().next() {
            if board.game_over {
                return;
            }
        }
        let window = windows.get_primary_mut().unwrap();
        if let Some(cursor) = window.cursor_position() {
            let cursor = cursor - Vec2::new(window.width(), window.height()) / 2.0;
            for basic_cell in cell_query.iter_mut() {
                if basic_cell.contains(cursor) {
                    if let Some(mut board) = board_query.iter_mut().next() {
                        let row = basic_cell.row;
                        let column = basic_cell.column;
                        if !board.initialized {
                            board.fill_board(40, (row, column)).unwrap();
                        }
                        let cell = &mut board.cells[row][column];
                        if cell.flagged || cell.opened {
                            break;
                        }

                        cell.opened = true;
                        ev_apply_material.send(ApplyMaterialEvent((row, column)));

                        if cell.mine {
                            ev_mine_clicked.send(MineClickedEvent);
                            break;
                        }

                        if cell.value == 0 {
                            ev_open_cells.send(ClearOpenCellsEvent((row, column)));
                            //break;
                        }

                        board.cells_unopened -= 1;
                        if board.cells_unopened == 0 {
                            ev_all_opened.send(AllCellsOpenedEvent);
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
                        if cell.opened {
                            break;
                        }
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
        app.add_system(left_click.system().label("left_click"));
        app.add_system(right_click.system());
    }
}
