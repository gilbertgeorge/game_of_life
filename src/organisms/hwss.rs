use crate::cell::Cell;
use super::Board;

pub fn generate(board: &mut Board, position: (usize, usize)) {
    let (column, row) = position;
    board.set_cell(row, column + 1, Cell::Alive);
    board.set_cell(row, column + 2, Cell::Alive);
    board.set_cell(row, column + 3, Cell::Alive);
    board.set_cell(row, column + 4, Cell::Alive);
    board.set_cell(row + 1, column, Cell::Alive);
    board.set_cell(row + 1, column + 4, Cell::Alive);
    board.set_cell(row + 2, column + 4, Cell::Alive);
    board.set_cell(row + 3, column, Cell::Alive);
    board.set_cell(row + 3, column + 3, Cell::Alive);

}