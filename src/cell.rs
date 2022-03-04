use crate::board::Board;
use bevy::ecs::component::Component;
use bevy::prelude::*;

pub const CELL_COLOR: bevy::prelude::Color = Color::MIDNIGHT_BLUE;

pub struct ApplyMaterialEvent(pub (usize, usize));

pub const SURROUND: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug)]
pub struct Cell {
    pub column: usize,
    pub flagged: bool,
    pub mine: bool,
    pub opened: bool,
    pub row: usize,
    pub value: u8,
}

pub struct NewCell {
    pub column: usize,
    pub position: Vec3,
    pub row: usize,
    pub size: Vec2,
}

#[derive(Component)]
pub struct BasicCell {
    pub column: usize,
    pub position: Vec3,
    pub row: usize,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
}

impl BasicCell {
    pub fn new(new_cell: NewCell) -> Self {
        let NewCell {
            column,
            position,
            row,
            size,
        } = new_cell;
        BasicCell {
            position,
            x0: position.x - size.x / 2.0,
            x1: position.x + size.x / 2.0,
            column,
            y0: position.y - size.y / 2.0,
            y1: position.y + size.y / 2.0,
            row,
        }
    }

    pub fn contains(&self, position: Vec2) -> bool {
        (self.x0..self.x1).contains(&position.x) && (self.y0..self.y1).contains(&position.y)
    }
}

fn apply_cell_material(
    asset_server: Res<AssetServer>,
    mut board_query: Query<&mut Board>,
    mut cell_query: Query<(&BasicCell, Entity, &mut Sprite)>,
    mut ev_apply_mat: EventReader<ApplyMaterialEvent>,
    mut commands: Commands,
) {
    let board = if let Some(b) = board_query.iter_mut().next() {
        b
    } else {
        return;
    };

    for ApplyMaterialEvent((row, col)) in ev_apply_mat.iter() {
        let row = *row;
        let col = *col;
        let (entity, mut sprite) = if let Some((_cell, entity, sprite)) = cell_query
            .iter_mut()
            .find(|(basic_cell, _, _)| basic_cell.row == row && basic_cell.column == col)
        {
            (entity, sprite)
        } else {
            return;
        };

        let cell = &board.cells[row][col];
        if cell.value == 0 {
            sprite.color = Color::GRAY;
        } else {
            sprite.color = Color::WHITE;
        }

        let texture = if cell.mine {
            asset_server.load("mine.png")
        } else {
            match cell.value {
                1 => asset_server.load("one.png"),
                2 => asset_server.load("two.png"),
                3 => asset_server.load("three.png"),
                4 => asset_server.load("four.png"),
                5 => asset_server.load("five.png"),
                6 => asset_server.load("six.png"),
                7 => asset_server.load("seven.png"),
                8 => asset_server.load("eight.png"),
                _ => Default::default(),
            }
        };
        let child = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(22.0, 22.0)),
                    ..Default::default()
                },

                texture,
                ..Default::default()
            })
            .id();

        commands.entity(entity).push_children(&[child]);
    }
}

impl Cell {
    pub fn surrounds(&self, cell_position: (usize, usize)) -> bool {
        SURROUND.iter().any(|(row, col)| {
            self.row as isize + row == cell_position.0 as isize
                && self.column as isize + col == cell_position.1 as isize
        })
    }
}

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyMaterialEvent>();
        app.add_system(apply_cell_material.after("left_click"));
    }
}
