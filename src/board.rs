use super::cell::Cell;

pub struct Board {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell::Dead; width * height],
            width,
            height
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> &Cell {
        let index = self.get_index(row, column);
        &self.cells[index]
    }

    pub fn get_next_board_state(&self) -> Board {
        let mut next = Board::new(self.width, self.height);

        for row in 0..self.height {
            for column in 0..self.width {
                let cell = self.get_cell(row, column);
                let live_neighbors = self.get_live_neighbor_count(row, column);
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (_, _) => Cell::Dead
                };
                next.set_cell(row, column, next_cell);
            }
        }

        next
    }

    pub fn set_cell(&mut self, row: usize, column: usize, cell: Cell) {
        let index = self.get_index(row, column);
        self.cells[index] = cell;
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                let index = self.get_index(neighbor_row, neighbor_column);
                count += match self.cells[index] {
                    Cell::Alive => 1,
                    Cell::Dead => 0
                };
            }
        }

        count
    }
}