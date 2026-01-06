#![allow(dead_code)]
use crate::block::BlockStruct;
use crate::position::Position;
pub struct LBlockStruct {
    pub block: BlockStruct,
}

impl LBlockStruct {
    pub fn new() -> Self {
        let mut block = BlockStruct::new();
        block.id = 1;
        block.cells.insert(
            0,
            vec![
                Position::new(0, 2),
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2),
            ],
        );
        block.cells.insert(
            1,
            vec![
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(2, 2),
            ],
        );
        block.cells.insert(
            2,
            vec![
                Position::new(1, 0),
                Position::new(1, 1),
                Position::new(1, 2),
                Position::new(2, 0),
            ],
        );
        block.cells.insert(
            3,
            vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ],
        );
        LBlockStruct {
            block: BlockStruct::new(),
        }
    }
}
