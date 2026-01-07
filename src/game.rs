#![allow(dead_code)]
use crate::block::*;
use crate::blocks::*;
use crate::grid::Grid;
pub struct Game<T> {
    pub grid: Grid,
    blocks: Vec<T>,
}

impl Game<T> {
    fn new() -> Self {
        todo!();
    }
}
