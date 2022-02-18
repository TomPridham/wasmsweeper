use crate::cell::{ApplyMaterialEvent, BasicCell, Cell, NewCell, CELL_COLOR, SURROUND};
use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::convert::TryFrom;
use std::error::Error;

pub struct ClearOpenCellsEvent(pub (usize, usize));
pub struct MineClickedEvent;
pub struct AllCellsOpenedEvent;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub cells_unopened: usize,
    pub game_over: bool,
    pub height: usize,
    pub initialized: bool,
    pub width: usize,
}

impl Board {
    pub fn check_in_bounds(
        &self,
        (cell_row, cell_col): (usize, usize),
        (row, col): (isize, isize),
    ) -> Option<(usize, usize)> {
        let r = cell_row as isize + row;
        let c = cell_col as isize + col;

        if (0..self.height as isize).contains(&r) && (0..self.width as isize).contains(&c) {
            return Some((r as usize, c as usize));
        }
        return None;
    }

    pub fn fill_board(&mut self, mines: u16, start: (usize, usize)) -> Result<(), Box<dyn Error>> {
        let board_size = self.height * self.width;

        if mines as usize >= board_size / 2 {
            return Err("You have requested too many mines for this size of board".into());
        }

        self.cells_unopened = board_size - mines as usize;
        self.initialized = true;

        let rows = self.height as usize;
        let columns = self.width as usize;
        let mut curr_mines = 0;

        let mut rng = SmallRng::from_entropy();

        while curr_mines < mines {
            let row = rng.gen_range(0..rows);
            let col = rng.gen_range(0..columns);
            let cell = &mut self.cells[row][col];
            if row == start.0 && col == start.1 || cell.surrounds((start.0, start.1)) {
                continue;
            }

            if !cell.mine {
                cell.mine = true;
                curr_mines += 1;
            }
        }

        (0..rows).for_each(|y| {
            (0..columns).for_each(|x| {
                let value = SURROUND.iter().fold(0, |acc, (surround_x, surround_y)| {
                    let surround_x = x as isize + surround_x;
                    let surround_y = y as isize + surround_y;

                    if (0..rows).contains(&usize::try_from(surround_y).unwrap_or(usize::MAX))
                        && (0..columns).contains(&usize::try_from(surround_x).unwrap_or(usize::MAX))
                    {
                        if self.cells[surround_y as usize][surround_x as usize].mine {
                            return acc + 1;
                        }
                    }
                    return acc;
                });
                self.cells[y][x].value = value;
            })
        });
        Ok(())
    }
}

pub fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_mine_clicked: EventReader<MineClickedEvent>,
    mut ev_all_opened: EventReader<AllCellsOpenedEvent>,
    mut board_query: Query<&mut Board>,
) {
    let mat = if ev_mine_clicked.iter().next().is_some() {
        asset_server.load("u_lose.png")
    } else if ev_all_opened.iter().next().is_some() {
        asset_server.load("u_win.png")
    } else {
        return;
    };

    if let Some(mut board) = board_query.iter_mut().next() {
        board.game_over = true;
    }

    let mut transform = Transform::from_xyz(0.0, 250.0, 1.0);
    transform.apply_non_uniform_scale(Vec3::new(3.0, 3.0, 3.0));
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(mat.into()),
        transform,
        ..Default::default()
    });
}

pub fn clear_open_cells(
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<&BasicCell>,
    mut ev_apply_material: EventWriter<ApplyMaterialEvent>,
    mut ev_open_cells: EventReader<ClearOpenCellsEvent>,
    mut ev_all_opened: EventWriter<AllCellsOpenedEvent>,
) {
    let mut board = if let Some(b) = board_query.iter_mut().next() {
        b
    } else {
        return;
    };

    let mut queue: Vec<(usize, usize)> = ev_open_cells
        .iter()
        .map(|ClearOpenCellsEvent((row, col))| (*row, *col))
        .collect();
    while queue.len() > 0 {
        let (curr_row, curr_col) = queue.pop().unwrap();
        SURROUND.iter().for_each(|(surround_row, surround_col)| {
            if let Some((valid_row, valid_col)) =
                board.check_in_bounds((curr_row, curr_col), (*surround_row, *surround_col))
            {
                let cell = &mut board.cells[valid_row][valid_col];
                if cell.opened {
                    return;
                }
                if cell.value == 0 {
                    queue.push((valid_row, valid_col));
                }

                cell.opened = true;
                board.cells_unopened -= 1;
                for basic_cell in cell_query.iter_mut() {
                    if basic_cell.row == valid_row && basic_cell.column == valid_col {
                        ev_apply_material.send(ApplyMaterialEvent((valid_row, valid_col)));

                        break;
                    }
                }
            }
        });
    }

    if board.cells_unopened == 0 {
        ev_all_opened.send(AllCellsOpenedEvent);
    }
}

pub fn generate_board(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let height = 16usize;
    let width = 16usize;
    let spacing = 2.0;
    let size = Vec2::new(22.0, 22.0);
    let mine_width = width as f32 * (size.x + spacing) - spacing;
    let offset = Vec3::new(
        -(mine_width - size.x) / 2.0,
        -(mine_width - size.y) / 2.0,
        0.0,
    );

    let cell_material = materials.add(CELL_COLOR.into());

    let cells: Vec<Vec<Cell>> = (0..height)
        .map(|row| {
            (0..width)
                .map(|column| {
                    let position = Vec3::new(
                        column as f32 * (size.x + spacing),
                        row as f32 * (size.y + spacing),
                        0.0,
                    ) + offset;
                    commands
                        .spawn_bundle(SpriteBundle {
                            material: cell_material.clone(),
                            sprite: Sprite::new(size),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        })
                        .insert(BasicCell::new(NewCell {
                            row,
                            column,
                            position,
                            size,
                        }));
                    Cell {
                        column,
                        flagged: false,
                        mine: false,
                        opened: false,
                        row,
                        value: 0,
                    }
                })
                .collect()
        })
        .collect();

    commands.spawn().insert(Board {
        cells,
        cells_unopened: usize::MAX,
        game_over: false,
        height,
        initialized: false,
        width,
    });
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ClearOpenCellsEvent>();
        app.add_event::<MineClickedEvent>();
        app.add_event::<AllCellsOpenedEvent>();
        app.add_startup_system(generate_board.system());
        app.add_system(clear_open_cells.system().after("left_click"));
        app.add_system(game_over.system().after("left_click"));
    }
}
