use tinycbg::{CyberGrindPattern, Tile};

#[test]
fn straight_line() {
    let mut pat = CyberGrindPattern::new();
    let mut line = pat.line((0, 0)..(0, 15));

    assert_eq!(line.len(), 16);
    line.set(Tile::with_height(10));

    for row in &pat {
        assert_eq!(row[0].height(), 10);
    }
}

#[test]
fn diagonal_line() {
    let mut pat = CyberGrindPattern::new();
    let mut line = pat.line((0, 0)..(15, 15));

    assert_eq!(line.len(), 16);
    line.set(Tile::with_height(10));

    for (i, row) in pat.into_iter().enumerate() {
        assert_eq!(row[i].height(), 10);
    }
}
