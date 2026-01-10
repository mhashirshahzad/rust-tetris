use crate::block::*;
use crate::grid::Grid;
use rand::prelude::*;
use raylib::consts::KeyboardKey;
use raylib::ffi::GetKeyPressed;
use std::mem;
pub struct Game {
    pub grid: Grid,
    blocks: Vec<BlockStruct>,
    curr_block: BlockStruct,
    next_block: BlockStruct,
}

impl Game {
    pub fn new() -> Self {
        let mut blocks = get_all_blocks();

        // Pick two random blocks for start
        let curr_block = Game::pick_random_block(&mut blocks);
        let next_block = Game::pick_random_block(&mut blocks);

        Game {
            grid: Grid::new(),
            blocks,
            curr_block,
            next_block,
        }
    }

    fn pick_random_block(blocks: &mut Vec<BlockStruct>) -> BlockStruct {
        let mut rng = rand::rng();
        if blocks.is_empty() {
            *blocks = get_all_blocks();
        }
        let idx = rng.random_range(0..blocks.len());
        blocks.remove(idx)
    }

    pub fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        self.grid.draw(d);
        self.curr_block.draw(d);
    }

    pub fn handle_input(&mut self) {
        unsafe {
            let key_pressed = GetKeyPressed();
            match key_pressed {
                k if k == KeyboardKey::KEY_LEFT as i32 => self.move_block_left(),
                k if k == KeyboardKey::KEY_RIGHT as i32 => self.move_block_right(),
                k if k == KeyboardKey::KEY_UP as i32 => self.rotate_block(),
                k if k == KeyboardKey::KEY_DOWN as i32 => self.move_block_down(),
                _ => {}
            }
        }
    }

    pub fn move_block_left(&mut self) {
        self.curr_block.move_blocks(0, -1);
        if self.is_block_outside() {
            self.curr_block.move_blocks(0, 1);
        }
    }

    pub fn move_block_right(&mut self) {
        self.curr_block.move_blocks(0, 1);
        if self.is_block_outside() {
            self.curr_block.move_blocks(0, -1)
        }
    }

    pub fn move_block_down(&mut self) {
        self.curr_block.move_blocks(1, 0);
        if self.is_block_outside() {
            self.curr_block.move_blocks(-1, 0);
            self.lock_block();
        }
    }

    pub fn move_block_up(&mut self) {
        self.curr_block.move_blocks(-1, 0);
        if self.is_block_outside() {
            self.curr_block.move_blocks(1, 0);
        }
    }
    fn is_block_outside(&self) -> bool {
        let tiles = self.curr_block.get_cell_positions();
        for item in tiles {
            if self.grid.is_cell_outside(item.row, item.col) {
                return true;
            }
        }
        return false;
    }
    fn rotate_block(&mut self) {
        self.curr_block.rotate();
        if self.is_block_outside() {
            self.curr_block.un_rotate();
        }
    }

    fn lock_block(&mut self) {
        let tiles = self.curr_block.get_cell_positions();

        for item in tiles {
            self.grid.grid[item.row as usize][item.col as usize] = self.curr_block.id;
        }

        self.curr_block = mem::replace(
            &mut self.next_block,
            Game::pick_random_block(&mut self.blocks),
        );
    }

    fn block_fits(&self) -> bool {
        let tiles = self.curr_block.get_cell_positions();
        for item in tiles {
            if self
                .grid
                .is_cell_empty(item.row as usize, item.col as usize)
                == false
            {
                return false;
            }
        }
        return true;
    }
}

fn get_all_blocks() -> Vec<BlockStruct> {
    vec![
        BlockStruct::new(BlockType::I),
        BlockStruct::new(BlockType::J),
        BlockStruct::new(BlockType::L),
        BlockStruct::new(BlockType::O),
        BlockStruct::new(BlockType::S),
        BlockStruct::new(BlockType::T),
        BlockStruct::new(BlockType::Z),
    ]
}
