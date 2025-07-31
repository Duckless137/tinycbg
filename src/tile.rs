use std::{
    fmt::{Debug, Formatter, Result as FmtRes},
    ops::{Add, Sub},
};

use crate::error::ParseErrorType;

/// A prefab is a spawning option for a tile.
/// There can only be one prefab per tile.
#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Prefab {
    /// Will spawn a projectile enemy on the tile
    Projectile,
    /// Will spawn a melee enemy on the tile
    Melee,
    /// Will spawn a Hideous Mass on the tile
    HideousMass,
    /// Will place a jump pad on the tile
    JumpPad,
    /// Will place stairs on the tile
    Stairs,
    /// Will not place/spawn anythin on the tile.
    /// Default when using `Tile::default()` or
    /// `CyberGrindPattern::new()`.
    #[default]
    None,
}

impl From<Prefab> for u8 {
    fn from(prefab: Prefab) -> Self {
        match prefab {
            Prefab::None => 0,
            Prefab::Melee => 1,
            Prefab::Projectile => 2,
            Prefab::HideousMass => 3,
            Prefab::JumpPad => 4,
            Prefab::Stairs => 5,
        }
    }
}

impl TryFrom<u8> for Prefab {
    type Error = ParseErrorType;
    fn try_from(byte: u8) -> Result<Self, ParseErrorType> {
        match byte {
            b'0' => Ok(Prefab::None),
            b'n' => Ok(Prefab::Melee),
            b'p' => Ok(Prefab::Projectile),
            b'H' => Ok(Prefab::HideousMass),
            b'J' => Ok(Prefab::JumpPad),
            b's' => Ok(Prefab::Stairs),
            _ => Err(ParseErrorType::InvalidPrefab),
        }
    }
}

/// A tile on the CyberGrindPattern.
/// Contains a height anywhere between
/// -50 and 50, and an optional prefab.
/// ```should_panic
/// use tinycbg::Prefab;
/// tinycbg::Tile::new(51, Prefab::None);
/// ```
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Tile {
    height: i8,
    prefab: Prefab,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        if self.prefab == Prefab::None {
            write!(f, "[{}]", self.height)
        } else {
            let letter = match self.prefab {
                Prefab::HideousMass => 'M',
                Prefab::Projectile => 'p',
                Prefab::Melee => 'm',
                Prefab::Stairs => 's',
                Prefab::JumpPad => 'j',
                Prefab::None => ' ',
            };

            write!(f, "[{}, {}]", self.height, letter)
        }
    }
}

impl Tile {
    fn check_height(height: i8) {
        assert!(height <= 50, "Height cannot be greater than 50");
        assert!(height >= -50, "Height cannot be less than -50");
    }

    /// Creates a new tile with height `height`
    /// and prefab `prefab`. Panics if `height`
    /// is greater than 50 or less than -50
    pub fn new(height: i8, prefab: Prefab) -> Self {
        Tile::check_height(height);
        Tile { height, prefab }
    }

    /// Creates a new tile with height
    /// `height` and no prefab.
    /// Panics if `height` is greater
    /// than 50 or less than -50.
    /// Equvalent to `Tile::from(height)`.
    pub fn with_height(height: i8) -> Self {
        Tile::check_height(height);
        Tile {
            height,
            prefab: Prefab::default(),
        }
    }

    /// Creates a new tile with prefab
    /// `prefab` and a height of 0.
    /// Equvalent to `Tile::from(prefab)`.
    pub fn with_prefab(prefab: Prefab) -> Self {
        Tile {
            height: i8::default(),
            prefab,
        }
    }

    /// Sets an existing tile's height to `new_height`.
    /// Panics if `new_height` is greater than 50 or less than -50.
    pub fn set_height(&mut self, new_height: i8) {
        Tile::check_height(new_height);
        self.height = new_height
    }

    /// Sets an existing tile's prefab to `new_prefab`.
    pub fn set_prefab(&mut self, new_prefab: Prefab) {
        self.prefab = new_prefab
    }

    /// Gets the height of a tile.
    pub fn height(self) -> i8 {
        self.height
    }

    /// Gets the prefab of a tile.
    pub fn prefab(self) -> Prefab {
        self.prefab
    }
}

impl From<i8> for Tile {
    fn from(height: i8) -> Self {
        Tile::check_height(height);
        Self {
            height,
            prefab: Prefab::default(),
        }
    }
}

impl From<Prefab> for Tile {
    fn from(prefab: Prefab) -> Self {
        Self {
            height: i8::default(),
            prefab,
        }
    }
}

/// Adds to the tile's height.
/// Will panic if `height` becomes
/// greater than 50 or less than -50
impl Add<i8> for Tile {
    type Output = Tile;
    fn add(self, rhs: i8) -> Self::Output {
        let new_height = self.height + rhs;
        Tile::check_height(new_height);
        Self {
            height: new_height,
            prefab: self.prefab,
        }
    }
}

/// Subtracts from the tile's height.
/// Will panic if `height` becomes
/// greater than 50 or less than -50
impl Sub<i8> for Tile {
    type Output = Tile;
    fn sub(self, rhs: i8) -> Self::Output {
        let new_height = self.height - rhs;
        Tile::check_height(new_height);
        Self {
            height: new_height,
            prefab: self.prefab,
        }
    }
}
