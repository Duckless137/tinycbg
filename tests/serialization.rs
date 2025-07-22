use tinycbg::*;

#[test]
fn write() {
    let mut pattern = CyberGrindPattern::new();
    let wall_tile = Tile::with_height(20);
    let enemy_tile = Tile::new(0, Prefab::Melee);

    pattern.copy_tile_to_row(wall_tile, 0);
    pattern.copy_tile_to_row(wall_tile, 15);

    pattern.copy_tile_to_column(wall_tile, 0);
    pattern.copy_tile_to_column(wall_tile, 15);

    pattern[(7, 7)] = enemy_tile;
    pattern[(8, 8)] = enemy_tile;

    pattern.write_to_path("tests/patterns/test.cgp").unwrap();
}
