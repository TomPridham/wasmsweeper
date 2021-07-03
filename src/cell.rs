use bevy::prelude::*;

pub struct BasicCell {
    pub column: usize,
    pub row: usize,
}

pub struct NewCell {
    pub column: usize,
    pub mine: bool,
    pub position: Vec3,
    pub row: usize,
    pub size: Vec2,
    pub value: u8,
}
#[derive(Debug)]
pub struct Cell {
    pub mine: bool,
    pub position: Vec3,
    pub value: u8,
    pub x: usize,
    pub y: usize,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
}

impl Cell {
    pub fn new(new_cell: NewCell) -> Cell {
        let NewCell {
            column,
            mine,
            position,
            row,
            size,
            value,
        } = new_cell;
        Cell {
            mine,
            position,
            value,
            x0: position.x - size.x / 2.0,
            x1: position.x + size.x / 2.0,
            x: column,
            y0: position.y - size.y / 2.0,
            y1: position.y + size.y / 2.0,
            y: row,
        }
    }

    pub fn contains(&self, position: Vec2) -> bool {
        (self.x0..self.x1).contains(&position.x) && (self.y0..self.y1).contains(&position.y)
    }
}
