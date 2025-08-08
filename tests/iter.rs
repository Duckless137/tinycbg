use tinycbg::{CyberGrindPattern, Prefab, Tile};

#[test]
fn iter_read() {
    let test_tile = Tile::new(10, Prefab::Stairs);
    let mut pat = CyberGrindPattern::new();

    for i in 0..16 {
        pat[(i, i)] = test_tile;
    }

    for (i, row) in pat.into_iter().enumerate() {
        assert!(row[i] == test_tile);
    }
}

#[test]
fn iter_write() {
    let test_tile = Tile::new(10, Prefab::Stairs);
    let mut pat = CyberGrindPattern::new();

    for (i, mut row) in (&mut pat).into_iter().enumerate() {
        row[i] = test_tile
    }

    for i in 0..16 {
        pat[(i, i)] = test_tile;
    }
}
