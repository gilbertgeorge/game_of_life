use crate::cell::Cell;
use super::Board;

pub fn generate(board: &mut Board, position: (usize, usize)) {
    let (column, row) = position;
    board.set_cell(row + 1, column + 1, Cell::Alive);
    board.set_cell(row + 1, column + 2, Cell::Alive);
    board.set_cell(row + 1, column + 3, Cell::Alive);
    board.set_cell(row + 2, column, Cell::Alive);
    board.set_cell(row + 2, column + 1, Cell::Alive);
    board.set_cell(row + 2, column + 2, Cell::Alive);
}