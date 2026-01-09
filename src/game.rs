use crate::block::*;
use crate::grid::Grid;
use rand::prelude::*;

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
