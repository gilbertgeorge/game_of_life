use crate::cell::Cell;
use super::Board;

pub fn generate(board: &mut Board, position: (usize, usize)) {
    let (column, row) = position;
    board.set_cell(row, column, Cell::Alive);
    board.set_cell(row, (column + 1) % board.width, Cell::Alive);
    board.set_cell(row, (column + 2) % board.width, Cell::Alive);
}