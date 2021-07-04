use bevy::prelude::*;

#[derive(Debug)]
pub struct Cell {
    pub column: usize,
    pub mine: bool,
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
}
