mod cell;
mod board;
mod organisms;

use board::Board;
use cell::Cell;
use organisms::OrganismType;

const WIDTH: usize = 150;
const HEIGHT: usize = 25;
const REFRESH_RATE: u64 = 30;

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);

    organisms::generate_organism(OrganismType::GosperGliderGun, &mut board, (3, 3));
    //organisms::generate_organism(OrganismType::Blinker, &mut board, (20, 20));
    //organisms::generate_organism(OrganismType::Beacon, &mut board, (70, 5));
    organisms::generate_organism(OrganismType::Toad, &mut board, (65, 5));
    organisms::generate_organism(OrganismType::GosperGliderGun, &mut board, (80, 10));
    //organisms::generate_organism(OrganismType::Glider, &mut board, (70, 20));

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
