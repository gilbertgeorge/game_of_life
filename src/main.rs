#[derive(Debug, Clone, Copy)]
enum Cell {
    Alive,
    Dead
}

struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell::Dead; width * height],
            width,
            height
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn get_cell(&self, row: usize, column: usize) -> &Cell {
        let index = self.get_index(row, column);
        &self.cells[index]
    }

    fn set_cell(&mut self, row: usize, column: usize, cell: Cell) {
        let index = self.get_index(row, column);
        self.cells[index] = cell;
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

    fn get_next_board_state(&self) -> Board {
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

    #[allow(unused)]
    fn generate_glider(&mut self, position: (usize, usize)) {
        let (row, column) = position;
        self.set_cell(row, column + 1, Cell::Alive);
        self.set_cell(row + 1, column + 2, Cell::Alive);
        self.set_cell(row + 2, column, Cell::Alive);
        self.set_cell(row + 2, column + 1, Cell::Alive);
        self.set_cell(row + 2, column + 2, Cell::Alive);
    }

    #[allow(unused)]
    fn generate_blinker(&mut self, position: (usize, usize)) {
        let (row, column) = position;
        self.set_cell(row, column, Cell::Alive);
        self.set_cell(row, column + 1, Cell::Alive);
        self.set_cell(row, column + 2, Cell::Alive);
    }

    #[allow(unused)]
    fn generate_toad(&mut self, position: (usize, usize)) {
        let (row, column) = position;
        self.set_cell(row + 1, column + 1, Cell::Alive);
        self.set_cell(row + 1, column + 2, Cell::Alive);
        self.set_cell(row + 1, column + 3, Cell::Alive);
        self.set_cell(row + 2, column, Cell::Alive);
        self.set_cell(row + 2, column + 1, Cell::Alive);
        self.set_cell(row + 2, column + 2, Cell::Alive);
    }

    #[allow(unused)]
    fn generate_beacon(&mut self, position: (usize, usize)) {
        let (row, column) = position;
        self.set_cell(row, column, Cell::Alive);
        self.set_cell(row, column + 1, Cell::Alive);
        self.set_cell(row + 1, column, Cell::Alive);
        self.set_cell(row + 1, column + 1, Cell::Alive);
        self.set_cell(row + 2, column + 2, Cell::Alive);
        self.set_cell(row + 2, column + 3, Cell::Alive);
        self.set_cell(row + 3, column + 2, Cell::Alive);
        self.set_cell(row + 3, column + 3, Cell::Alive);
    }

    #[allow(unused)]
    fn generate_gosper_glider_gun(&mut self, position: (usize, usize)) {
        let (row, column) = position;
        self.set_cell(row, column + 24, Cell::Alive);
        self.set_cell(row + 1, column + 22, Cell::Alive);
        self.set_cell(row + 1, column + 24, Cell::Alive);
        self.set_cell(row + 2, column + 12, Cell::Alive);
        self.set_cell(row + 2, column + 13, Cell::Alive);
        self.set_cell(row + 2, column + 20, Cell::Alive);
        self.set_cell(row + 2, column + 21, Cell::Alive);
        self.set_cell(row + 2, column + 34, Cell::Alive);
        self.set_cell(row + 2, column + 35, Cell::Alive);
        self.set_cell(row + 3, column + 11, Cell::Alive);
        self.set_cell(row + 3, column + 15, Cell::Alive);
        self.set_cell(row + 3, column + 20, Cell::Alive);
        self.set_cell(row + 3, column + 21, Cell::Alive);
        self.set_cell(row + 3, column + 34, Cell::Alive);
        self.set_cell(row + 3, column + 35, Cell::Alive);
        self.set_cell(row + 4, column + 0, Cell::Alive);
        self.set_cell(row + 4, column + 1, Cell::Alive);
        self.set_cell(row + 4, column + 10, Cell::Alive);
        self.set_cell(row + 4, column + 16, Cell::Alive);
        self.set_cell(row + 4, column + 20, Cell::Alive);
        self.set_cell(row + 4, column + 21, Cell::Alive);
        self.set_cell(row + 5, column + 0, Cell::Alive);
        self.set_cell(row + 5, column + 1, Cell::Alive);
        self.set_cell(row + 5, column + 10, Cell::Alive);
        self.set_cell(row + 5, column + 14, Cell::Alive);
        self.set_cell(row + 5, column + 16, Cell::Alive);
        self.set_cell(row + 5, column + 17, Cell::Alive);
        self.set_cell(row + 5, column + 22, Cell::Alive);
        self.set_cell(row + 5, column + 24, Cell::Alive);
        self.set_cell(row + 6, column + 10, Cell::Alive);
        self.set_cell(row + 6, column + 16, Cell::Alive);
        self.set_cell(row + 6, column + 24, Cell::Alive);
        self.set_cell(row + 7, column + 11, Cell::Alive);
        self.set_cell(row + 7, column + 15, Cell::Alive);
        self.set_cell(row + 8, column + 12, Cell::Alive);
        self.set_cell(row + 8, column + 13, Cell::Alive);
    }
}


fn main() {
    let mut board = Board::new(80, 25);

    //board.generate_glider((3, 3));
    //board.generate_blinker((3, 3));
    //board.generate_toad((3, 3));
    //board.generate_beacon((3, 3));
    board.generate_gosper_glider_gun((3, 3));

    loop {
        print!("\x1B[2J\x1B[1;1H");
        board = board.get_next_board_state();
        for row in 0..board.height {
            for column in 0..board.width {
                let cell = board.get_cell(row, column);
                let symbol = match cell {
                    Cell::Alive => '◼',
                    Cell::Dead => ' '
                };
                print!("{}", symbol);
            }
            print!("\n");
        }

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
