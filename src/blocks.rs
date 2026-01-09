#![allow(dead_code)]
use crate::block::BlockStruct;
use crate::position::Position;
use std::collections::HashMap;
pub struct LBlockStruct {
    pub block: BlockStruct,
}

pub fn l_block() -> (i32, HashMap<i32, Vec<Position>>) {
    let id = 1;
    let mut cells: HashMap<i32, Vec<Position>> = HashMap::new();

    cells.insert(
        0,
        vec![
            Position::new(0, 2),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
        ],
    );

    cells.insert(
        1,
        vec![
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(2, 2),
        ],
    );

    cells.insert(
        2,
        vec![
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 0),
        ],
    );

    cells.insert(
        3,
        vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
    );

    (id, cells)
}

pub fn j_block() -> (i32, HashMap<i32, Vec<Position>>) {
    let id = 2;
    let mut cells: HashMap<i32, Vec<Position>> = HashMap::new();

    cells.insert(
        0,
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
        ],
    );

    cells.insert(
        1,
        vec![
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
    );

    cells.insert(
        2,
        vec![
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 2),
        ],
    );

    cells.insert(
        3,
        vec![
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 0),
            Position::new(2, 1),
        ],
    );

    (id, cells)
}

pub fn i_block() -> (i32, HashMap<i32, Vec<Position>>) {
    let id = 3;
    let mut cells: HashMap<i32, Vec<Position>> = HashMap::new();

    cells.insert(
        0,
        vec![
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(1, 3),
        ],
    );

    cells.insert(
        1,
        vec![
            Position::new(0, 2),
            Position::new(1, 2),
            Position::new(2, 2),
            Position::new(3, 2),
        ],
    );

    cells.insert(
        2,
        vec![
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(2, 2),
            Position::new(2, 3),
        ],
    );

    cells.insert(
        3,
        vec![
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(3, 1),
        ],
    );

    (id, cells)
}

pub fn s_block() -> (i32, HashMap<i32, Vec<Position>>) {
    let id = 5;
    let mut cells: HashMap<i32, Vec<Position>> = HashMap::new();

    cells.insert(
        0,
        vec![
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(1, 0),
            Position::new(1, 1),
        ],
    );

    cells.insert(
        1,
        vec![
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 2),
        ],
    );

    cells.insert(
        2,
        vec![
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 0),
            Position::new(2, 1),
        ],
    );

    cells.insert(
        3,
        vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
    );

    (id, cells)
}

pub fn t_block() -> (i32, HashMap<i32, Vec<Position>>) {
    let id = 6;
    let mut cells: HashMap<i32, Vec<Position>> = HashMap::new();

    cells.insert(
        0,
        vec![
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
        ],
    );

    cells.insert(
        1,
        vec![
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 1),
        ],
    );

    cells.insert(
        2,
        vec![
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 1),
        ],
    );

    cells.insert(
        3,
        vec![
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(2, 1),
        ],
    );

    (id, cells)
}

pub fn o_block() -> (i32, HashMap<i32, Vec<Position>>) {
    let id = 4;
    let mut cells: HashMap<i32, Vec<Position>> = HashMap::new();

    cells.insert(
        0,
        vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 1),
        ],
    );

    (id, cells)
}

pub fn z_block() -> (i32, HashMap<i32, Vec<Position>>) {
    let id = 7;
    let mut cells: HashMap<i32, Vec<Position>> = HashMap::new();

    cells.insert(
        0,
        vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(1, 2),
        ],
    );

    cells.insert(
        1,
        vec![
            Position::new(0, 2),
            Position::new(1, 1),
            Position::new(1, 2),
            Position::new(2, 1),
        ],
    );

    cells.insert(
        2,
        vec![
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(2, 2),
        ],
    );

    cells.insert(
        3,
        vec![
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(2, 0),
        ],
    );

    (id, cells)
}
//
// impl LBlockStruct {
//     pub fn new() -> Self {
//         let mut block = BlockStruct::new();
//         block.id = 1;
//         block.cells.insert(
//             0,
//             vec![
//                 Position::new(0, 2),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//             ],
//         );
//         block.cells.insert(
//             1,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 1),
//                 Position::new(2, 1),
//                 Position::new(2, 2),
//             ],
//         );
//         block.cells.insert(
//             2,
//             vec![
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(2, 0),
//             ],
//         );
//         block.cells.insert(
//             3,
//             vec![
//                 Position::new(0, 0),
//                 Position::new(0, 1),
//                 Position::new(1, 1),
//                 Position::new(2, 1),
//             ],
//         );
//         block.move_blocks(0, 3);
//
//         LBlockStruct { block }
//     }
// }
//
// struct JBlockStruct {
//     block: BlockStruct,
// }
//
// impl JBlockStruct {
//     fn new() -> Self {
//         let mut block = BlockStruct::new();
//         block.id = 2;
//         block.cells.insert(
//             0,
//             vec![
//                 Position::new(0, 0),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//             ],
//         );
//         block.cells.insert(
//             1,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(0, 2),
//                 Position::new(1, 1),
//                 Position::new(2, 1),
//             ],
//         );
//         block.cells.insert(
//             2,
//             vec![
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(2, 2),
//             ],
//         );
//         block.cells.insert(
//             3,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 1),
//                 Position::new(2, 0),
//                 Position::new(2, 1),
//             ],
//         );
//         block.move_blocks(0, 3);
//
//         JBlockStruct { block }
//     }
// }
// pub struct IBlockStruct {
//     pub block: BlockStruct,
// }
//
// impl IBlockStruct {
//     pub fn new() -> Self {
//         let mut block = BlockStruct::new();
//         block.id = 3;
//
//         block.cells.insert(
//             0,
//             vec![
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(1, 3),
//             ],
//         );
//
//         block.cells.insert(
//             1,
//             vec![
//                 Position::new(0, 2),
//                 Position::new(1, 2),
//                 Position::new(2, 2),
//                 Position::new(3, 2),
//             ],
//         );
//
//         block.cells.insert(
//             2,
//             vec![
//                 Position::new(2, 0),
//                 Position::new(2, 1),
//                 Position::new(2, 2),
//                 Position::new(2, 3),
//             ],
//         );
//
//         block.cells.insert(
//             3,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 1),
//                 Position::new(2, 1),
//                 Position::new(3, 1),
//             ],
//         );
//
//         block.move_blocks(-1, 3);
//         Self { block }
//     }
// }
//
// pub struct OBlockStruct {
//     pub block: BlockStruct,
// }
//
// impl OBlockStruct {
//     pub fn new() -> Self {
//         let mut block = BlockStruct::new();
//         block.id = 4;
//
//         block.cells.insert(
//             0,
//             vec![
//                 Position::new(0, 0),
//                 Position::new(0, 1),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//             ],
//         );
//
//         block.move_blocks(1, 3);
//         Self { block }
//     }
// }
//
// pub struct SBlockStruct {
//     pub block: BlockStruct,
// }
//
// impl SBlockStruct {
//     pub fn new() -> Self {
//         let mut block = BlockStruct::new();
//         block.id = 5;
//
//         block.cells.insert(
//             0,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(0, 2),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//             ],
//         );
//
//         block.cells.insert(
//             1,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(2, 2),
//             ],
//         );
//
//         block.cells.insert(
//             2,
//             vec![
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(2, 0),
//                 Position::new(2, 1),
//             ],
//         );
//
//         block.cells.insert(
//             3,
//             vec![
//                 Position::new(0, 0),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(2, 1),
//             ],
//         );
//
//         block.move_blocks(0, 3);
//         Self { block }
//     }
// }
//
// pub struct TBlockStruct {
//     pub block: BlockStruct,
// }
//
// impl TBlockStruct {
//     pub fn new() -> Self {
//         let mut block = BlockStruct::new();
//         block.id = 6;
//
//         block.cells.insert(
//             0,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//             ],
//         );
//
//         block.cells.insert(
//             1,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(2, 1),
//             ],
//         );
//
//         block.cells.insert(
//             2,
//             vec![
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(2, 1),
//             ],
//         );
//
//         block.cells.insert(
//             3,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(2, 1),
//             ],
//         );
//         block.move_blocks(0, 3);
//         Self { block }
//     }
// }
//
// pub struct ZBlockStruct {
//     pub block: BlockStruct,
// }

// impl ZBlockStruct {
//     pub fn new() -> Self {
//         let mut block = BlockStruct::new();
//         block.id = 7;
//
//         block.cells.insert(
//             0,
//             vec![
//                 Position::new(0, 0),
//                 Position::new(0, 1),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//             ],
//         );
//
//         block.cells.insert(
//             1,
//             vec![
//                 Position::new(0, 2),
//                 Position::new(1, 1),
//                 Position::new(1, 2),
//                 Position::new(2, 1),
//             ],
//         );
//
//         block.cells.insert(
//             2,
//             vec![
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(2, 1),
//                 Position::new(2, 2),
//             ],
//         );
//
//         block.cells.insert(
//             3,
//             vec![
//                 Position::new(0, 1),
//                 Position::new(1, 0),
//                 Position::new(1, 1),
//                 Position::new(2, 0),
//             ],
//         );
//         block.move_blocks(0, 3);
//
//         Self { block }
//     }
// }
