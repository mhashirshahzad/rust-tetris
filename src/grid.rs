#![allow(dead_code)]
use crate::colors;
use raylib::prelude::*;
pub struct Grid {
    rows: i32,
    cols: i32,
    cell_size: i32,
    pub grid: [[i32; 10]; 20],
    colors: Vec<Color>,
}

impl Grid {
    /// Creates a new grid and initializes it.
    pub fn new() -> Self {
        let mut grid = [[0; 10]; 20]; // array[20][10]
        for row in 0..20 {
            for col in 0..10 {
                grid[row][col] = 0;
            }
        }

        Grid {
            rows: 20,
            cols: 10,
            cell_size: 30,
            grid,
            colors: colors::get_cell_colors(),
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell_val = self.grid[row as usize][col as usize];
                d.draw_rectangle(
                    col * self.cell_size + 1,
                    row * self.cell_size + 1,
                    self.cell_size - 1,
                    self.cell_size - 1,
                    self.colors[cell_val as usize],
                );
            }
        }
    }
    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.grid[row as usize][col as usize]);
            }
            print!("\n");
        }
    }
}
