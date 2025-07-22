use tinycbg::*;

#[test]
#[ignore = "only needs to be run once"]
fn create_test_patterns() {
    let mut pattern = CyberGrindPattern::new();

    for i in 0..=100 {
        let height = i as i8 - 50;
        pattern[i].set_height(height);
    }

    pattern[251].set_prefab(Prefab::Melee);
    pattern[252].set_prefab(Prefab::Projectile);
    pattern[253].set_prefab(Prefab::HideousMass);
    pattern[254].set_prefab(Prefab::JumpPad);
    pattern[255].set_prefab(Prefab::Stairs);

    pattern
        .write_to_path("tests/patterns/valid/range.cgp")
        .unwrap();

    for i in 0..256 {
        pattern[i].set_height(-50);
        pattern[i].set_prefab(Prefab::Melee);
    }

    pattern
        .write_to_path("tests/patterns/valid/max_len.cgp")
        .unwrap();
}

#[test]
fn parse() {
    let pattern = CyberGrindPattern::parse_path("tests/patterns/valid/range.cgp").unwrap();
    for i in 0..=100 {
        println!("{:?}", pattern[i]);
        assert_eq!(pattern[i].height(), i as i8 - 50);
        assert_eq!(pattern[i].prefab(), Prefab::None);
    }

    assert_eq!(pattern[251].prefab(), Prefab::Melee);
    assert_eq!(pattern[252].prefab(), Prefab::Projectile);
    assert_eq!(pattern[253].prefab(), Prefab::HideousMass);
    assert_eq!(pattern[254].prefab(), Prefab::JumpPad);
    assert_eq!(pattern[255].prefab(), Prefab::Stairs);

    let pattern = CyberGrindPattern::parse_path("tests/patterns/valid/max_len.cgp").unwrap();
    for i in 0..256 {
        assert_eq!(pattern[i].height(), -50);
        assert_eq!(pattern[i].prefab(), Prefab::Melee);
    }
}
