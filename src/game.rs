use crate::block::*;
use crate::grid::Grid;
use rand::prelude::*;
use raylib::consts::KeyboardKey;
use raylib::ffi::{
    GetKeyPressed, InitAudioDevice, LoadMusicStream, LoadSound, Music, PlayMusicStream, PlaySound,
    Sound,
};
use std::ffi::CString;
use std::mem;
pub struct Game {
    pub grid: Grid,
    blocks: Vec<BlockStruct>,
    curr_block: BlockStruct,
    next_block: BlockStruct,
    pub game_over: bool,
    pub score: i32,
    pub music: Music,
    rot_sound: Sound,
    clear_sound: Sound,
}

impl Game {
    pub fn new() -> Self {
        let mut blocks = get_all_blocks();

        // Pick two random blocks for start
        let curr_block = Game::pick_random_block(&mut blocks);
        let next_block = Game::pick_random_block(&mut blocks);

        let music_path = CString::new("sounds/music.mp3").unwrap();
        let rot_path = CString::new("sounds/rot.mp3").unwrap();
        let clear_path = CString::new("sounds/clear_sound.mp3").unwrap();

        let music = unsafe { LoadMusicStream(music_path.as_ptr()) };
        let rot_sound = unsafe { LoadSound(rot_path.as_ptr()) };
        let clear_sound = unsafe { LoadSound(clear_path.as_ptr()) };
        unsafe { InitAudioDevice() };
        unsafe { PlayMusicStream(music) };
        Game {
            rot_sound,
            clear_sound,
            music,
            grid: Grid::new(),
            blocks,
            curr_block,
            next_block,
            game_over: false,
            score: 0,
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
        self.curr_block.draw(d, 11, 11);
        match self.next_block.id {
            3 => {
                self.next_block.draw(d, 255, 290);
            }
            4 => self.next_block.draw(d, 255, 280),
            _ => self.next_block.draw(d, 270, 270),
        }
    }

    pub fn handle_input(&mut self) {
        unsafe {
            let key_pressed = GetKeyPressed();
            match key_pressed {
                k if k == KeyboardKey::KEY_LEFT as i32 => self.move_block_left(),
                k if k == KeyboardKey::KEY_RIGHT as i32 => self.move_block_right(),
                k if k == KeyboardKey::KEY_UP as i32 => self.rotate_block(),
                k if k == KeyboardKey::KEY_DOWN as i32 => {
                    self.move_block_down();
                    self.update_score(0, 1);
                }
                k if self.game_over == true && k != 0 => {
                    self.game_over = false;
                    self.reset();
                }
                _ => {}
            }
        }
    }

    pub fn move_block_left(&mut self) {
        if self.game_over {
            return ();
        }
        self.curr_block.move_blocks(0, -1);
        if self.is_block_outside() || self.block_fits() == false {
            self.curr_block.move_blocks(0, 1);
        }
    }

    pub fn move_block_right(&mut self) {
        if self.game_over {
            return ();
        }
        self.curr_block.move_blocks(0, 1);
        if self.is_block_outside() || self.block_fits() == false {
            self.curr_block.move_blocks(0, -1)
        }
    }

    pub fn move_block_down(&mut self) {
        if self.game_over {
            return ();
        }
        self.curr_block.move_blocks(1, 0);
        if self.is_block_outside() || self.block_fits() == false {
            self.curr_block.move_blocks(-1, 0);
            self.lock_block();
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
        if self.game_over {
            return ();
        }
        self.curr_block.rotate();
        if self.is_block_outside() || self.block_fits() == false {
            self.curr_block.un_rotate();
        } else {
            unsafe {
                PlaySound(self.rot_sound);
            }
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

        if self.block_fits() == false {
            println!("[  Game  ] Game Over!");
            self.game_over = true;
        }

        let rows_cleared = self.grid.clear_full_rows_v2();
        if rows_cleared > 0 {
            unsafe {
                PlaySound(self.clear_sound);
            }
            self.update_score(rows_cleared as i32, 0);
        }
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
    fn reset(&mut self) {
        self.grid = Grid::new();
        self.blocks = get_all_blocks();
        self.curr_block = Game::pick_random_block(&mut self.blocks);
        self.score = 0
    }

    fn update_score(&mut self, lines_cleared: i32, moved_down_points: i32) {
        match lines_cleared {
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 500,
            _ => {}
        }
        self.score += moved_down_points;
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
