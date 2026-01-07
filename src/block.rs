#![allow(dead_code)]
use crate::colors;
use crate::position::Position;
use raylib::color::Color;
use raylib::prelude::*;
use std::collections::HashMap;

use crate::position;
pub struct BlockStruct {
    pub id: i32,
    pub cells: HashMap<i32, Vec<position::Position>>,
    cell_size: i32,
    rot_state: i32,
    colors: Vec<Color>,
    row_offset: i32,
    col_offset: i32,
}

impl BlockStruct {
    pub fn new() -> Self {
        BlockStruct {
            id: 0,
            cells: HashMap::new(),
            cell_size: 30,
            rot_state: 0,
            colors: colors::get_cell_colors(),
            row_offset: 0,
            col_offset: 0,
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let tiles = self.get_cell_positions();
        for item in tiles {
            d.draw_rectangle(
                item.col * self.cell_size + 1,
                item.row * self.cell_size + 1,
                self.cell_size - 1,
                self.cell_size - 1,
                self.colors[self.id as usize],
            );
        }
    }
    pub fn move_blocks(&mut self, rows: i32, cols: i32) {
        self.row_offset += rows;
        self.col_offset += cols;
    }

    pub fn get_cell_positions(&self) -> Vec<Position> {
        let tiles = &self.cells[&self.rot_state];
        let mut moved_tiles: Vec<Position> = Vec::new();

        for item in tiles {
            let new_pos = Position {
                row: item.row + self.row_offset,
                col: item.col + self.col_offset,
            };
            moved_tiles.push(new_pos);
        }
        moved_tiles
    }
}
