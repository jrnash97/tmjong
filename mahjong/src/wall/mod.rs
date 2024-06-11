#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use crate::tiles::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, PartialEq, Eq)]
pub struct DeadWall {
    kan_draws: VecDeque<Tile>,
    dora_indicators: Vec<Tile>,
    ura_dora_indicators: Vec<Tile>,
}

impl DeadWall {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.kan_draws.len() + self.dora_indicators.len() + self.ura_dora_indicators.len()
    }
}

impl Default for DeadWall {
    fn default() -> Self {
        Self {
            kan_draws: VecDeque::with_capacity(4),
            dora_indicators: Vec::with_capacity(5),
            ura_dora_indicators: Vec::with_capacity(5),
        }
    }
}

impl From<VecDeque<Tile>> for DeadWall {
    fn from(value: VecDeque<Tile>) -> Self {
        println!("in = {value:#?}");
        let mut kan_draws = value;
        let mut dora_indicators: Vec<Tile> = Vec::with_capacity(5);
        let mut ura_dora_indicators: Vec<Tile> = Vec::with_capacity(5);
        let mut dora_and_ura_dora = kan_draws.split_off(4);

        for n in 0..dora_and_ura_dora.len() {
            if n % 2 == 0 {
                dora_indicators.push(dora_and_ura_dora.pop_front().unwrap());
            } else {
                ura_dora_indicators.push(dora_and_ura_dora.pop_front().unwrap());
            }
        }

        // println!("kan_draws.len() = {}", kan_draws.len());
        // println!("dora_indicators = {dora_indicators:#?}");
        // println!("ura_dora_indicators = {ura_dora_indicators:#?}");

        Self {
            kan_draws,
            dora_indicators,
            ura_dora_indicators,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Wall {
    pub tiles: VecDeque<Tile>,
}

impl Wall {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shuffle(mut self) -> Self {
        self.tiles.make_contiguous().shuffle(&mut thread_rng());
        self
    }

    pub fn split_dead_wall(&mut self, break_roll: usize) -> DeadWall {
        self.tiles.rotate_left(Self::rotate_to_index(break_roll));
        let dead_tiles = self.tiles.split_off(self.tiles.len() - 14);
        DeadWall::from(dead_tiles)
    }

    fn rotate_to_index(break_roll: usize) -> usize {
        // break_roll is an index between 2 and 12 (incl.)
        let wall_len = 136;
        let side_len = wall_len / 4;

        ((wall_len - (((break_roll - 1) % 4) * side_len)) + (break_roll * 2)) % wall_len
    }
}

impl Default for Wall {
    fn default() -> Self {
        let mut tiles = VecDeque::with_capacity(136);

        for tile in [
            Tile::Pin(1),
            Tile::Pin(1),
            Tile::Pin(1),
            Tile::Pin(1),
            Tile::Pin(2),
            Tile::Pin(2),
            Tile::Pin(2),
            Tile::Pin(2),
            Tile::Pin(3),
            Tile::Pin(3),
            Tile::Pin(3),
            Tile::Pin(3),
            Tile::Pin(4),
            Tile::Pin(4),
            Tile::Pin(4),
            Tile::Pin(4),
            Tile::Pin(5),
            Tile::Pin(5),
            Tile::Pin(5),
            Tile::Pin(5),
            Tile::Pin(6),
            Tile::Pin(6),
            Tile::Pin(6),
            Tile::Pin(6),
            Tile::Pin(7),
            Tile::Pin(7),
            Tile::Pin(7),
            Tile::Pin(7),
            Tile::Pin(8),
            Tile::Pin(8),
            Tile::Pin(8),
            Tile::Pin(8),
            Tile::Pin(9),
            Tile::Pin(9),
            Tile::Pin(9),
            Tile::Pin(9),
            Tile::Man(1),
            Tile::Man(1),
            Tile::Man(1),
            Tile::Man(1),
            Tile::Man(2),
            Tile::Man(2),
            Tile::Man(2),
            Tile::Man(2),
            Tile::Man(3),
            Tile::Man(3),
            Tile::Man(3),
            Tile::Man(3),
            Tile::Man(4),
            Tile::Man(4),
            Tile::Man(4),
            Tile::Man(4),
            Tile::Man(5),
            Tile::Man(5),
            Tile::Man(5),
            Tile::Man(5),
            Tile::Man(6),
            Tile::Man(6),
            Tile::Man(6),
            Tile::Man(6),
            Tile::Man(7),
            Tile::Man(7),
            Tile::Man(7),
            Tile::Man(7),
            Tile::Man(8),
            Tile::Man(8),
            Tile::Man(8),
            Tile::Man(8),
            Tile::Man(9),
            Tile::Man(9),
            Tile::Man(9),
            Tile::Man(9),
            Tile::Sou(1),
            Tile::Sou(1),
            Tile::Sou(1),
            Tile::Sou(1),
            Tile::Sou(2),
            Tile::Sou(2),
            Tile::Sou(2),
            Tile::Sou(2),
            Tile::Sou(3),
            Tile::Sou(3),
            Tile::Sou(3),
            Tile::Sou(3),
            Tile::Sou(4),
            Tile::Sou(4),
            Tile::Sou(4),
            Tile::Sou(4),
            Tile::Sou(5),
            Tile::Sou(5),
            Tile::Sou(5),
            Tile::Sou(5),
            Tile::Sou(6),
            Tile::Sou(6),
            Tile::Sou(6),
            Tile::Sou(6),
            Tile::Sou(7),
            Tile::Sou(7),
            Tile::Sou(7),
            Tile::Sou(7),
            Tile::Sou(8),
            Tile::Sou(8),
            Tile::Sou(8),
            Tile::Sou(8),
            Tile::Sou(9),
            Tile::Sou(9),
            Tile::Sou(9),
            Tile::Sou(9),
            Tile::Wind(Wind::East),
            Tile::Wind(Wind::East),
            Tile::Wind(Wind::East),
            Tile::Wind(Wind::East),
            Tile::Wind(Wind::South),
            Tile::Wind(Wind::South),
            Tile::Wind(Wind::South),
            Tile::Wind(Wind::South),
            Tile::Wind(Wind::West),
            Tile::Wind(Wind::West),
            Tile::Wind(Wind::West),
            Tile::Wind(Wind::West),
            Tile::Wind(Wind::North),
            Tile::Wind(Wind::North),
            Tile::Wind(Wind::North),
            Tile::Wind(Wind::North),
            Tile::Dragon(Dragon::Green),
            Tile::Dragon(Dragon::Green),
            Tile::Dragon(Dragon::Green),
            Tile::Dragon(Dragon::Green),
            Tile::Dragon(Dragon::Red),
            Tile::Dragon(Dragon::Red),
            Tile::Dragon(Dragon::Red),
            Tile::Dragon(Dragon::Red),
            Tile::Dragon(Dragon::White),
            Tile::Dragon(Dragon::White),
            Tile::Dragon(Dragon::White),
            Tile::Dragon(Dragon::White),
        ] {
            tiles.push_back(tile);
        }

        Self { tiles }
    }
}
