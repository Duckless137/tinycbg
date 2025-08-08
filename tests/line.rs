use tinycbg::{CyberGrindPattern, Tile};

#[test]
fn straight_line() {
    let mut pat = CyberGrindPattern::new();
    let mut line = pat.line((0, 0)..(0, 7));

    line.set(Tile::with_height(10));
}
