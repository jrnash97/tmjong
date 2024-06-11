#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Dragon {
    Green,
    Red,
    White,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Pin(u8),
    Man(u8),
    Sou(u8),
    Wind(Wind),
    Dragon(Dragon),
}

// Returns the Dora tile given a specific indicator
pub fn get_dora(indicator: &Tile) -> Tile {
    match indicator {
        Tile::Pin(num) => {
            if *num == 9 {
                Tile::Pin(1)
            } else {
                Tile::Pin(num + 1)
            }
        }
        Tile::Man(num) => {
            if *num == 9 {
                Tile::Man(1)
            } else {
                Tile::Man(num + 1)
            }
        }
        Tile::Sou(num) => {
            if *num == 9 {
                Tile::Sou(1)
            } else {
                Tile::Sou(num + 1)
            }
        }
        Tile::Wind(wind) => Tile::Wind(match wind {
            Wind::East => Wind::South,
            Wind::South => Wind::West,
            Wind::West => Wind::North,
            Wind::North => Wind::East,
        }),
        Tile::Dragon(dragon) => Tile::Dragon(match dragon {
            Dragon::Green => Dragon::Red,
            Dragon::Red => Dragon::White,
            Dragon::White => Dragon::Green,
        }),
    }
}
