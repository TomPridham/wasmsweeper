use super::cell::{BasicCell, Cell, NewCell};
use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::convert::TryFrom;
use std::error::Error;

// Create small, cheap to initialize and fast RNG with a random seed.
// The randomness is supplied by the operating system.
const SURROUND: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub initialized: bool,
    pub height: usize,
    pub width: usize,
}

impl Board {
    pub fn fill_board(&mut self, mines: u16, start: (usize, usize)) -> Result<(), Box<dyn Error>> {
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
            if row == start.0 && col == start.1 {
                continue;
            }
            let cell = &mut self.cells[row][col];
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
                    if surround_x < 0
                        || surround_y < 0
                        || surround_x >= rows as isize
                        || surround_y >= columns as isize
                    {
                        return acc;
                    }
                    if self.cells[usize::try_from(surround_x).unwrap_or(0)]
                        [usize::try_from(surround_y).unwrap_or(0)]
                    .mine
                    {
                        return acc + 1;
                    }
                    return acc;
                });
                self.cells[y][x].value = value;
            })
        });
        Ok(())
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

    let mine_material = materials.add(Color::INDIGO.into());
    let mm = materials.add(Color::RED.into());

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
                            material: if column % 2 == 0 {
                                mine_material.clone()
                            } else {
                                mm.clone()
                            },
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
                        mine: false,
                        value: 0,
                        row,
                        column,
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
        app.add_startup_system(generate_board.system());
    }
}
