use super::*;

#[test]
fn dragon_indicator_to_dora() {
    let tile = Tile::Dragon(Dragon::Red);
    assert_eq!(get_dora(&tile), Tile::Dragon(Dragon::White));
}

#[test]
fn wind_indicator_to_dora() {
    let tile = Tile::Wind(Wind::East);
    assert_eq!(get_dora(&tile), Tile::Wind(Wind::South));
}

#[test]
fn number_indicator_to_dora() {
    let tile = Tile::Man(1);
    assert_eq!(get_dora(&tile), Tile::Man(2));
}

#[test]
fn number_indicator_to_dora_wrap_around() {
    let tile = Tile::Man(9);
    assert_eq!(get_dora(&tile), Tile::Man(1));
}
