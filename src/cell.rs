use bevy::prelude::*;

pub const CELL_COLOR: bevy::prelude::Color = Color::MIDNIGHT_BLUE;

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

    pub fn apply_material(
        &self,
        asset_server: &Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mat_handle: &mut Handle<ColorMaterial>,
        mine: bool,
        value: u8,
    ) {
        let material = if mine {
            asset_server.load("mine.png").into()
        } else {
            match value {
                1 => asset_server.load("one.png").into(),
                2 => asset_server.load("two.png").into(),
                3 => asset_server.load("three.png").into(),
                4 => asset_server.load("four.png").into(),
                5 => asset_server.load("five.png").into(),
                6 => asset_server.load("six.png").into(),
                7 => asset_server.load("seven.png").into(),
                8 => asset_server.load("eight.png").into(),
                _ => Color::GRAY.into(),
            }
        };
        *mat_handle = materials.add(material);
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
