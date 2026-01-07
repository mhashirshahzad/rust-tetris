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
}

impl BlockStruct {
    pub fn new() -> Self {
        BlockStruct {
            id: 0,
            cells: HashMap::new(),
            cell_size: 30,
            rot_state: 0,
            colors: colors::get_cell_colors(),
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if let Some(tiles) = self.cells.get(&self.rot_state) {
            for item in tiles {
                d.draw_rectangle(
                    item.col * self.cell_size + 1,
                    item.row * self.cell_size + 1,
                    self.cell_size - 1,
                    self.cell_size - 1,
                    self.colors[self.id as usize],
                );
            }
        } else {
            eprintln!("Error getting tiles");
        }
    }
}
