mod glider;
mod beacon;
mod blinker;
mod toad;
mod gosper_glider_gun;

use super::board::Board;

#[allow(unused)]
pub enum OrganismType {
    Glider,
    Beacon,
    Blinker,
    Toad,
    GosperGliderGun
}

pub fn generate_organism(organism_type: OrganismType, board: &mut Board, position: (usize, usize)) {
    match organism_type {
        OrganismType::Glider => glider::generate(board, position),
        OrganismType::Beacon => beacon::generate(board, position),
        OrganismType::Blinker => blinker::generate(board, position),
        OrganismType::Toad => toad::generate(board, position),
        OrganismType::GosperGliderGun => gosper_glider_gun::generate(board, position),
    }
}