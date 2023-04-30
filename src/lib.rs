extern crate wasm_bindgen;
extern crate cfg_if;

mod utils;
mod cell;
mod board;
mod organisms;

use std::fmt;
use wasm_bindgen::prelude::*;
use cell::Cell;
use board::Board;
use organisms::OrganismType;

#[wasm_bindgen]
pub struct Universe {
    board: Board,
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.board = self.board.get_next_board_state();
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            board: Board::start(width, height, cells),
        }
    }

    pub fn generate_organism(&mut self, organism_type: OrganismType, row: u32, column: u32) {
        organisms::generate_organism(organism_type, &mut self.board, (row as usize, column as usize));
    }

    pub fn clear(&mut self) {
        self.board.clear();
    }

    pub fn width(&self) -> u32 {
        self.board.width as u32
    }

    pub fn height(&self) -> u32 {
        self.board.height as u32
    }

    pub fn cells(&self) -> *const Cell {
        self.board.cells.as_ptr()
    }

    pub fn get_cell(&self, row: u32, column: u32) -> Cell {
        *self.board.get_cell(row as usize, column as usize)
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.board.get_index(row as usize, column as usize);
        self.board.cells[idx].toggle();
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.board.cells.as_slice().chunks(self.board.width) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
