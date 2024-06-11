use super::Wall;

#[test]
fn total_tiles_in_wall_is_136() {
    let wall = Wall::new();
    assert_eq!(wall.tiles.len(), 136);
}

#[test]
fn new_walls_are_equal() {
    let wall_a = Wall::new();
    let wall_b = Wall::new();

    assert_eq!(wall_a, wall_b);
}

#[test]
fn shuffled_walls_are_shuffled() {
    let control_wall = Wall::new();
    let shuffled_wall = Wall::new().shuffle();

    assert_ne!(shuffled_wall, control_wall);
}

#[test]
fn shuffled_walls_are_different() {
    let wall_a = Wall::new().shuffle();
    let wall_b = Wall::new().shuffle();

    assert_ne!(wall_a, wall_b);
}

#[test]
fn shuffled_wall_has_same_total_tiles() {
    let control_wall = Wall::new();
    let shuffled_wall = Wall::new().shuffle();

    assert_eq!(shuffled_wall.tiles.len(), control_wall.tiles.len());
}

#[test]
fn roll_2_to_rotate_wall() {
    let break_roll = 2;
    assert_eq!(Wall::rotate_to_index(break_roll), 106);
}

#[test]
fn roll_3_to_rotate_wall() {
    let break_roll = 3;
    assert_eq!(Wall::rotate_to_index(break_roll), 74);
}

#[test]
fn roll_4_to_rotate_wall() {
    let break_roll = 4;
    assert_eq!(Wall::rotate_to_index(break_roll), 42);
}

#[test]
fn roll_5_to_rotate_wall() {
    let break_roll = 5;
    assert_eq!(Wall::rotate_to_index(break_roll), 10);
}

#[test]
fn roll_6_to_rotate_wall() {
    let break_roll = 6;
    assert_eq!(Wall::rotate_to_index(break_roll), 114);
}

#[test]
fn roll_7_to_rotate_wall() {
    let break_roll = 7;
    assert_eq!(Wall::rotate_to_index(break_roll), 82);
}

#[test]
fn roll_8_to_rotate_wall() {
    let break_roll = 8;
    assert_eq!(Wall::rotate_to_index(break_roll), 50);
}

#[test]
fn roll_9_to_rotate_wall() {
    let break_roll = 9;
    assert_eq!(Wall::rotate_to_index(break_roll), 18);
}

#[test]
fn roll_10_to_rotate_wall() {
    let break_roll = 10;
    assert_eq!(Wall::rotate_to_index(break_roll), 122);
}

#[test]
fn roll_11_to_rotate_wall() {
    let break_roll = 11;
    assert_eq!(Wall::rotate_to_index(break_roll), 90);
}

#[test]
fn roll_12_to_rotate_wall() {
    let break_roll = 12;
    assert_eq!(Wall::rotate_to_index(break_roll), 58);
}

#[test]
fn rotated_wall_rotates() {
    let break_roll = 7;
    let control_wall = Wall::new();
    let mut test_wall = Wall::new();
    let _ = test_wall.split_dead_wall(break_roll);

    assert_eq!(test_wall.tiles.front(), control_wall.tiles.get(82));
}

#[test]
fn live_wall_correct_total_tiles() {
    let mut wall = Wall::new();
    let _ = wall.split_dead_wall(2);

    assert_eq!(wall.tiles.len(), 136 - 14);
}

#[test]
fn dead_wall_correct_total_tiles() {
    let dead_wall = Wall::new().split_dead_wall(2);
    assert_eq!(dead_wall.len(), 14);
}
