use bevy::prelude::*;

pub struct NewCell {
    pub column: u16,
    pub mine: bool,
    pub offset: Vec3,
    pub position: Vec3,
    pub row: u16,
    pub size: Vec2,
    pub value: u8,
}
#[derive(Debug)]
pub struct Cell {
    pub mine: bool,
    pub value: u8,
    pub x: u16,
    pub y: u16,
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
            offset,
            position,
            row,
            size,
            value,
        } = new_cell;
        Cell {
            y: row,
            x: column,
            mine,
            value,
            x0: position.x - size.x / 2.0,
            x1: position.x + size.x / 2.0,
            y0: position.y - size.y / 2.0,
            y1: position.y + size.y / 2.0,
        }
    }
    pub fn contains(&self, position: Vec2) -> bool {
        (self.x0..self.x1).contains(&position.x) && (self.y0..self.y1).contains(&position.y)
    }
}
