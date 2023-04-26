// use crate::cell::Cell;
// use crate::board::Board;
//
// #[allow(unused)]
// pub mod glider {
//     use super::Cell;
//     use super::Board;
//     pub fn generate(board: &mut Board, position: (usize, usize)) {
//         let (row, column) = position;
//         board.set_cell(row, column + 1, Cell::Alive);
//         board.set_cell(row + 1, column + 2, Cell::Alive);
//         board.set_cell(row + 2, column, Cell::Alive);
//         board.set_cell(row + 2, column + 1, Cell::Alive);
//         board.set_cell(row + 2, column + 2, Cell::Alive);
//     }
// }
//
// #[allow(unused)]
// pub mod beacon {
//     use super::Cell;
//     use super::Board;
//     pub fn generate(board: &mut Board, position: (usize, usize)) {
//         let (row, column) = position;
//         board.set_cell(row, column, Cell::Alive);
//         board.set_cell(row, column + 1, Cell::Alive);
//         board.set_cell(row + 1, column, Cell::Alive);
//         board.set_cell(row + 1, column + 1, Cell::Alive);
//         board.set_cell(row + 2, column + 2, Cell::Alive);
//         board.set_cell(row + 2, column + 3, Cell::Alive);
//         board.set_cell(row + 3, column + 2, Cell::Alive);
//         board.set_cell(row + 3, column + 3, Cell::Alive);
//     }
// }
//
// #[allow(unused)]
// pub mod blinker {
//     use super::Cell;
//     use super::Board;
//     pub fn generate(board: &mut Board, position: (usize, usize)) {
//         let (row, column) = position;
//         board.set_cell(row, column, Cell::Alive);
//         board.set_cell(row, column + 1, Cell::Alive);
//         board.set_cell(row, column + 2, Cell::Alive);
//     }
// }
//
// #[allow(unused)]
// pub mod toad {
//     use super::Cell;
//     use super::Board;
//     pub fn generate(board: &mut Board, position: (usize, usize)) {
//         let (row, column) = position;
//         board.set_cell(row + 1, column + 1, Cell::Alive);
//         board.set_cell(row + 1, column + 2, Cell::Alive);
//         board.set_cell(row + 1, column + 3, Cell::Alive);
//         board.set_cell(row + 2, column, Cell::Alive);
//         board.set_cell(row + 2, column + 1, Cell::Alive);
//         board.set_cell(row + 2, column + 2, Cell::Alive);
//     }
// }
//
// #[allow(unused)]
// pub mod gosper_glider_gun{
//     use super::Cell;
//     use super::Board;
//     pub fn generate(board: &mut Board, position: (usize, usize)) {
//         let (row, column) = position;
//         board.set_cell(row, column + 24, Cell::Alive);
//         board.set_cell(row + 1, column + 22, Cell::Alive);
//         board.set_cell(row + 1, column + 24, Cell::Alive);
//         board.set_cell(row + 2, column + 12, Cell::Alive);
//         board.set_cell(row + 2, column + 13, Cell::Alive);
//         board.set_cell(row + 2, column + 20, Cell::Alive);
//         board.set_cell(row + 2, column + 21, Cell::Alive);
//         board.set_cell(row + 2, column + 34, Cell::Alive);
//         board.set_cell(row + 2, column + 35, Cell::Alive);
//         board.set_cell(row + 3, column + 11, Cell::Alive);
//         board.set_cell(row + 3, column + 15, Cell::Alive);
//         board.set_cell(row + 3, column + 20, Cell::Alive);
//         board.set_cell(row + 3, column + 21, Cell::Alive);
//         board.set_cell(row + 3, column + 34, Cell::Alive);
//         board.set_cell(row + 3, column + 35, Cell::Alive);
//         board.set_cell(row + 4, column, Cell::Alive);
//         board.set_cell(row + 4, column + 1, Cell::Alive);
//         board.set_cell(row + 4, column + 10, Cell::Alive);
//         board.set_cell(row + 4, column + 16, Cell::Alive);
//         board.set_cell(row + 4, column + 20, Cell::Alive);
//         board.set_cell(row + 4, column + 21, Cell::Alive);
//         board.set_cell(row + 5, column, Cell::Alive);
//         board.set_cell(row + 5, column + 1, Cell::Alive);
//         board.set_cell(row + 5, column + 10, Cell::Alive);
//         board.set_cell(row + 5, column + 14, Cell::Alive);
//         board.set_cell(row + 5, column + 16, Cell::Alive);
//         board.set_cell(row + 5, column + 17, Cell::Alive);
//         board.set_cell(row + 5, column + 22, Cell::Alive);
//         board.set_cell(row + 5, column + 24, Cell::Alive);
//         board.set_cell(row + 6, column + 10, Cell::Alive);
//         board.set_cell(row + 6, column + 16, Cell::Alive);
//         board.set_cell(row + 6, column + 24, Cell::Alive);
//         board.set_cell(row + 7, column + 11, Cell::Alive);
//         board.set_cell(row + 7, column + 15, Cell::Alive);
//         board.set_cell(row + 8, column + 12, Cell::Alive);
//         board.set_cell(row + 8, column + 13, Cell::Alive);
//     }
// }
//
// //
// // #[allow(unused)]
// // pub fn generate_glider(board: &mut Board, position: (usize, usize)) {
// //     let (row, column) = position;
// //     self.set_cell(row, column + 1, Cell::Alive);
// //     self.set_cell(row + 1, column + 2, Cell::Alive);
// //     self.set_cell(row + 2, column, Cell::Alive);
// //     self.set_cell(row + 2, column + 1, Cell::Alive);
// //     self.set_cell(row + 2, column + 2, Cell::Alive);
// // }
// //
// // #[allow(unused)]
// // pub fn generate_blinker(&mut self, position: (usize, usize)) {
// //     let (row, column) = position;
// //     self.set_cell(row, column, Cell::Alive);
// //     self.set_cell(row, column + 1, Cell::Alive);
// //     self.set_cell(row, column + 2, Cell::Alive);
// // }
// //
// // #[allow(unused)]
// // pub fn generate_toad(&mut self, position: (usize, usize)) {
// //     let (row, column) = position;
// //     self.set_cell(row + 1, column + 1, Cell::Alive);
// //     self.set_cell(row + 1, column + 2, Cell::Alive);
// //     self.set_cell(row + 1, column + 3, Cell::Alive);
// //     self.set_cell(row + 2, column, Cell::Alive);
// //     self.set_cell(row + 2, column + 1, Cell::Alive);
// //     self.set_cell(row + 2, column + 2, Cell::Alive);
// // }
// //
// // #[allow(unused)]
// // pub fn generate_beacon(&mut self, position: (usize, usize)) {
// //     let (row, column) = position;
// //     self.set_cell(row, column, Cell::Alive);
// //     self.set_cell(row, column + 1, Cell::Alive);
// //     self.set_cell(row + 1, column, Cell::Alive);
// //     self.set_cell(row + 1, column + 1, Cell::Alive);
// //     self.set_cell(row + 2, column + 2, Cell::Alive);
// //     self.set_cell(row + 2, column + 3, Cell::Alive);
// //     self.set_cell(row + 3, column + 2, Cell::Alive);
// //     self.set_cell(row + 3, column + 3, Cell::Alive);
// // }
// //
// // #[allow(unused)]
// // pub fn generate_gosper_glider_gun(&mut self, position: (usize, usize)) {
// //     let (row, column) = position;
// //     self.set_cell(row, column + 24, Cell::Alive);
// //     self.set_cell(row + 1, column + 22, Cell::Alive);
// //     self.set_cell(row + 1, column + 24, Cell::Alive);
// //     self.set_cell(row + 2, column + 12, Cell::Alive);
// //     self.set_cell(row + 2, column + 13, Cell::Alive);
// //     self.set_cell(row + 2, column + 20, Cell::Alive);
// //     self.set_cell(row + 2, column + 21, Cell::Alive);
// //     self.set_cell(row + 2, column + 34, Cell::Alive);
// //     self.set_cell(row + 2, column + 35, Cell::Alive);
// //     self.set_cell(row + 3, column + 11, Cell::Alive);
// //     self.set_cell(row + 3, column + 15, Cell::Alive);
// //     self.set_cell(row + 3, column + 20, Cell::Alive);
// //     self.set_cell(row + 3, column + 21, Cell::Alive);
// //     self.set_cell(row + 3, column + 34, Cell::Alive);
// //     self.set_cell(row + 3, column + 35, Cell::Alive);
// //     self.set_cell(row + 4, column + 0, Cell::Alive);
// //     self.set_cell(row + 4, column + 1, Cell::Alive);
// //     self.set_cell(row + 4, column + 10, Cell::Alive);
// //     self.set_cell(row + 4, column + 16, Cell::Alive);
// //     self.set_cell(row + 4, column + 20, Cell::Alive);
// //     self.set_cell(row + 4, column + 21, Cell::Alive);
// //     self.set_cell(row + 5, column + 0, Cell::Alive);
// //     self.set_cell(row + 5, column + 1, Cell::Alive);
// //     self.set_cell(row + 5, column + 10, Cell::Alive);
// //     self.set_cell(row + 5, column + 14, Cell::Alive);
// //     self.set_cell(row + 5, column + 16, Cell::Alive);
// //     self.set_cell(row + 5, column + 17, Cell::Alive);
// //     self.set_cell(row + 5, column + 22, Cell::Alive);
// //     self.set_cell(row + 5, column + 24, Cell::Alive);
// //     self.set_cell(row + 6, column + 10, Cell::Alive);
// //     self.set_cell(row + 6, column + 16, Cell::Alive);
// //     self.set_cell(row + 6, column + 24, Cell::Alive);
// //     self.set_cell(row + 7, column + 11, Cell::Alive);
// //     self.set_cell(row + 7, column + 15, Cell::Alive);
// //     self.set_cell(row + 8, column + 12, Cell::Alive);
// //     self.set_cell(row + 8, column + 13, Cell::Alive);
// // }