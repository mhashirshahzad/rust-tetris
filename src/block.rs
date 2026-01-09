#![allow(dead_code)]
use crate::position::Position;
use crate::{blocks, colors};
use raylib::color::Color;
use raylib::prelude::*;
use std::collections::HashMap;

use crate::position;

pub struct BlockStruct {
    pub block_type: BlockType,
    pub id: i32,
    pub cells: HashMap<i32, Vec<position::Position>>,
    cell_size: i32,
    rot_state: i32,
    colors: Vec<Color>,
    row_offset: i32,
    col_offset: i32,
}

#[derive(Clone, Copy)]
pub enum BlockType {
    O,
    J,
    L,
    I,
    S,
    T,
    Z,
}

impl BlockStruct {
    pub fn new(block_type: BlockType) -> Self {
        let (id, cells) = match block_type {
            BlockType::O => blocks::o_block(),
            BlockType::I => blocks::i_block(),
            BlockType::T => blocks::t_block(),
            BlockType::S => blocks::s_block(),
            BlockType::Z => blocks::z_block(),
            BlockType::L => blocks::l_block(),
            BlockType::J => blocks::j_block(),
        };

        let mut block = BlockStruct {
            block_type,
            id,
            cells,
            cell_size: 30,
            rot_state: 0,
            colors: colors::get_cell_colors(),
            row_offset: 0,
            col_offset: 0,
        };

        match block_type {
            BlockType::O => block.move_blocks(1, 3),
            BlockType::J => block.move_blocks(1, 3),
            BlockType::T => block.move_blocks(0, 3),
            BlockType::Z => block.move_blocks(0, 3),
            BlockType::I => block.move_blocks(0, 3),
            BlockType::L => block.move_blocks(0, 3),
            BlockType::S => block.move_blocks(0, 3),
        }
        return block;
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
