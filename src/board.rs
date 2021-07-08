use crate::cell::{ApplyMaterialEvent, BasicCell, Cell, NewCell, CELL_COLOR, SURROUND};
use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::convert::TryFrom;
use std::error::Error;

pub struct ClearOpenCellsEvent(pub (usize, usize));

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub initialized: bool,
    pub height: usize,
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
        self.initialized = true;
        let mut rng = SmallRng::from_entropy();
        if mines as usize >= self.height * self.width / 2 {
            return Err("You have requested too many mines for this size of board".into());
        }

        let rows = self.height as usize;
        let columns = self.width as usize;
        let mut curr_mines = 0;

        while curr_mines < mines {
            let row = rng.gen_range(0..rows);
            let col = rng.gen_range(0..columns);
            let cell = &mut self.cells[row][col];
            if row == start.0 && col == start.1 || cell.surrounds((row, col)) {
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

pub fn clear_open_cells(
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<&BasicCell>,
    mut ev_apply_material: EventWriter<ApplyMaterialEvent>,
    mut ev_open_cells: EventReader<ClearOpenCellsEvent>,
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
                for basic_cell in cell_query.iter_mut() {
                    if basic_cell.row == valid_row && basic_cell.column == valid_col {
                        ev_apply_material.send(ApplyMaterialEvent((valid_row, valid_col)));

                        break;
                    }
                }
            }
        });
    }
}

pub fn generate_board(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let height = 4usize;
    let width = 4usize;
    let spacing = 2.0;
    let size = Vec2::new(96.0, 96.0);
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
        initialized: false,
        height,
        width,
    });
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ClearOpenCellsEvent>();
        app.add_startup_system(generate_board.system());
        app.add_system(clear_open_cells.system().after("left_click"));
    }
}
