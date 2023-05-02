mod glider;
mod beacon;
mod blinker;
mod toad;
mod glider_gun;
mod hwss;

use super::board::Board;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum OrganismType {
    Glider,
    Beacon,
    Blinker,
    Toad,
    GliderGun,
    HWSS,
}

pub fn generate_organism(organism_type: OrganismType, board: &mut Board, position: (usize, usize)) {
    match organism_type {
        OrganismType::Glider => glider::generate(board, position),
        OrganismType::Beacon => beacon::generate(board, position),
        OrganismType::Blinker => blinker::generate(board, position),
        OrganismType::Toad => toad::generate(board, position),
        OrganismType::GliderGun => glider_gun::generate(board, position),
        OrganismType::HWSS => hwss::generate(board, position),
    }
}