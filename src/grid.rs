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

    pub fn is_cell_outside(&self, row: i32, col: i32) -> bool {
        if row >= 0 && row < self.rows && col >= 0 && col < self.cols {
            return false;
        }
        return true;
    }

    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        if self.grid[row][col] == 0 {
            return true;
        }
        return false;
    }

    pub fn clear_full_rows(&mut self) -> usize {
        let mut completed: usize = 0;
        for row in 0..self.rows {
            if self.is_row_full(row as usize) {
                self.clear_row(row as usize);
                completed += 1;
            } else if completed > 0 {
                self.move_row_down(row as usize, completed);
            }
        }
        return completed;
    }
    pub fn clear_full_rows_v2(&mut self) -> usize {
        let mut cleared = 0;
        let mut write_row = self.rows as isize - 1;

        for read_row in (0..self.rows).rev() {
            if self.is_row_full(read_row as usize) {
                cleared += 1;
            } else {
                if write_row != read_row as isize {
                    self.grid[write_row as usize] = self.grid[read_row as usize].clone();
                }
                write_row -= 1;
            }
        }

        for row in 0..=write_row {
            for col in 0..self.cols {
                self.grid[row as usize][col as usize] = 0;
            }
        }

        cleared
    }

    fn is_row_full(&self, row: usize) -> bool {
        for col in 0..self.cols {
            if self.grid[row][col as usize] == 0 {
                return false;
            }
        }
        return true;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..self.cols {
            self.grid[row][col as usize] = 0;
        }
    }

    fn move_row_down(&mut self, row: usize, num_rows: usize) {
        for col in 0..self.cols {
            self.grid[row + num_rows][col as usize] = self.grid[row][col as usize];
            self.grid[row][col as usize] = 0;
        }
    }
}
