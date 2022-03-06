use crate::board::{
    AllCellsOpenedEvent, Board, ChordSolvedCellEvent, ClearOpenCellsEvent, FlagSolvedCellEvent,
    MineClickedEvent,
};
use crate::cell::{ApplyMaterialEvent, BasicCell, CELL_COLOR};
use bevy::prelude::*;

pub fn left_click(
    mouse_button_input: Res<Input<MouseButton>>,
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<&BasicCell>,
    mut ev_all_opened: EventWriter<AllCellsOpenedEvent>,
    mut ev_apply_material: EventWriter<ApplyMaterialEvent>,
    mut ev_chord_cell: EventWriter<ChordSolvedCellEvent>,
    mut ev_mine_clicked: EventWriter<MineClickedEvent>,
    mut ev_open_cells: EventWriter<ClearOpenCellsEvent>,
    mut windows: ResMut<Windows>,
) {
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }
    let mut board = board_query.single_mut();

    if board.game_over {
        return;
    }

    let window = windows.get_primary_mut().unwrap();
    let cursor = if let Some(cursor) = window.cursor_position() {
        cursor - Vec2::new(window.width(), window.height()) / 2.0
    } else {
        return;
    };
    for basic_cell in cell_query.iter_mut() {
        if !basic_cell.contains(cursor) {
            continue;
        }
        let row = basic_cell.row;
        let column = basic_cell.column;
        if !board.initialized {
            board.fill_board(40, (row, column)).unwrap();
        }

        let cell = &mut board.cells[row][column];

        if cell.flagged {
            return;
        }

        if cell.opened {
            ev_chord_cell.send(ChordSolvedCellEvent((row, column)));
            return;
        }

        cell.opened = true;
        ev_apply_material.send(ApplyMaterialEvent((row, column)));

        if cell.mine {
            ev_mine_clicked.send(MineClickedEvent);
            return;
        }

        if cell.value == 0 {
            ev_open_cells.send(ClearOpenCellsEvent((row, column)));
        }

        board.cells_unopened -= 1;
        if board.cells_unopened == 0 {
            ev_all_opened.send(AllCellsOpenedEvent);
        }
    }
}

pub fn right_click(
    asset_server: Res<AssetServer>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<(&BasicCell, Entity, &mut Sprite)>,
    mut commands: Commands,
    mut ev_flag_cell: EventWriter<FlagSolvedCellEvent>,
    mut windows: ResMut<Windows>,
) {
    if !mouse_button_input.just_released(MouseButton::Right) {
        return;
    }

    let mut board = board_query.single_mut();
    if board.game_over {
        return;
    }

    let window = windows.get_primary_mut().unwrap();
    let cursor = if let Some(cursor) = window.cursor_position() {
        cursor - Vec2::new(window.width(), window.height()) / 2.0
    } else {
        return;
    };
    for (basic_cell, entity, mut sprite) in cell_query.iter_mut() {
        if !basic_cell.contains(cursor) {
            continue;
        }
        let row = basic_cell.row;
        let column = basic_cell.column;
        let cell = &mut board.cells[row][column];
        if cell.opened {
            ev_flag_cell.send(FlagSolvedCellEvent((row, column)));
            return;
        }

        cell.flagged = !cell.flagged;

        if cell.flagged {
            let child = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(22.0, 22.0)),
                        ..Default::default()
                    },

                    texture: asset_server.load("flag.png"),
                    ..Default::default()
                })
                .id();

            commands.entity(entity).push_children(&[child]);
            sprite.color = Color::WHITE;
        } else {
            sprite.color = CELL_COLOR;
            commands.entity(entity).despawn_descendants();
        }
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(left_click.label("left_click"));
        app.add_system(right_click.label("right_click"));
    }
}
