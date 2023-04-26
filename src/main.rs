mod cell;
mod board;
//mod organism;
mod organisms;

use board::Board;
use cell::Cell;
use organisms::OrganismType;
//use crate::organism::{blinker, gosper_glider_gun};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const REFRESH_RATE: u64 = 20;

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);

    // gosper_glider_gun::generate(&mut board, (3, 3));
    // blinker::generate(&mut board, (20, 20));
    organisms::generate_organism(OrganismType::Glider, &mut board, (3, 3));

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

        std::thread::sleep(std::time::Duration::from_millis(REFRESH_RATE));
    }
}
