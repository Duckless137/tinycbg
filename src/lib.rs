pub mod error;
mod iter;
mod normal_fmt;
mod tile;
pub use tile::Prefab;
pub use tile::Tile;

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::ops::RangeFull;

pub const MAX_FILE_SIZE: usize = 1569;
pub mod prelude {
    pub use crate::error::IoError;
    pub use crate::error::ParseError;
    pub use crate::error::ParseErrorType;
    pub use crate::CyberGrindPattern;
}

impl Default for CyberGrindPattern {
    fn default() -> Self {
        CyberGrindPattern {
            tiles: [Tile::default(); 256],
        }
    }
}

impl From<Vec<Tile>> for CyberGrindPattern {
    fn from(values: Vec<Tile>) -> Self {
        let mut new = Self::new();
        for (i, value) in values.iter().enumerate() {
            if i > 255 {
                break;
            }
            new[i] = *value;
        }
        new
    }
}
impl From<[Tile; 256]> for CyberGrindPattern {
    fn from(tiles: [Tile; 256]) -> Self {
        CyberGrindPattern { tiles }
    }
}

#[derive(Clone)]
pub struct CyberGrindPattern {
    tiles: [Tile; 256],
}

/// An interface for building cybergrind patterns.
/// This a 512-byte wide struct with a padding of
/// 16 bits, so small enough to store on the stack
/// if necessary.
///
/// A Cybergrind pattern is a 16x16 array
/// of tiles. Each tile has a height and
/// optional prefab.
/// ```
/// use tinycbg::{CyberGrindPattern, Tile, Prefab};
///
/// let mut pat = CyberGrindPattern::new();
/// let wall_tile = Tile::new(20, Prefab::None);
///
/// pat.copy_tile_to_column(wall_tile, 0);
/// pat.copy_tile_to_column(wall_tile, 15);
/// // Patterns can be indexed with
/// // coordinates or numbers 0-255
/// pat[(7, 7)].set_prefab(Prefab::Melee);
///
/// pat.write_to_path("test.cgp"); // Cybergrind patterns are stored in '.cgp' files
/// ```
impl CyberGrindPattern {
    /// Creates a new Cybergrind pattern
    /// with all tiles initialized to have
    /// a height of zero and no prefab.
    /// Same as `CyberGrindPattern::default()`
    /// ```
    /// use tinycbg::{CyberGrindPattern, Prefab};
    ///
    /// let pat = CyberGrindPattern::new();
    /// assert_eq!(pat[0].height(), 0);
    /// assert_eq!(pat[0].prefab(), Prefab::None);
    /// ```
    pub fn new() -> Self {
        CyberGrindPattern::default()
    }

    /// Copies the data from tile `Tile` to
    /// row number `row`.
    /// ```
    /// use tinycbg::{CyberGrindPattern, Tile};
    ///
    /// let mut pat = CyberGrindPattern::new();
    ///
    /// let wall_tile = Tile::with_height(20);
    ///
    /// let row = 10;
    /// pat.copy_tile_to_row(wall_tile, row);
    /// for i in 0..16 {
    ///     assert_eq!(pat[(i, row)], wall_tile);
    /// }
    /// ```
    pub fn copy_tile_to_row(&mut self, tile: Tile, row: usize) {
        for i in 0..16 {
            self[i + row * 16] = tile;
        }
    }

    /// Copies the data from tile `Tile` to
    /// column number `column`.
    /// ```
    /// use tinycbg::{CyberGrindPattern, Tile};
    ///
    /// let mut pat = CyberGrindPattern::new();
    ///
    /// let wall_tile = Tile::with_height(20);
    ///
    /// let column = 7;
    /// pat.copy_tile_to_column(wall_tile, column);
    /// for i in 0..16 {
    ///     assert_eq!(pat[(column, i)], wall_tile);
    ///
    /// }
    /// ```
    pub fn copy_tile_to_column(&mut self, tile: Tile, column: usize) {
        for i in 0..16 {
            self[i * 16 + column] = tile;
        }
    }
}

impl Index<usize> for CyberGrindPattern {
    type Output = Tile;
    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl IndexMut<usize> for CyberGrindPattern {
    fn index_mut(&mut self, index: usize) -> &mut Tile {
        &mut self.tiles[index]
    }
}

impl Index<(usize, usize)> for CyberGrindPattern {
    type Output = Tile;
    fn index(&self, coordinate: (usize, usize)) -> &Self::Output {
        let index = coordinate.1 * 16 + coordinate.0;
        &self.tiles[index]
    }
}

impl IndexMut<(usize, usize)> for CyberGrindPattern {
    fn index_mut(&mut self, coordinate: (usize, usize)) -> &mut Tile {
        let index = coordinate.1 * 16 + coordinate.0;
        &mut self.tiles[index]
    }
}

impl Index<Range<usize>> for CyberGrindPattern {
    type Output = [Tile];
    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.tiles[range]
    }
}

impl IndexMut<Range<usize>> for CyberGrindPattern {
    fn index_mut(&mut self, range: Range<usize>) -> &mut [Tile] {
        &mut self.tiles[range]
    }
}

impl Index<RangeFull> for CyberGrindPattern {
    type Output = [Tile];
    fn index(&self, range: RangeFull) -> &Self::Output {
        &self.tiles[range]
    }
}

impl IndexMut<RangeFull> for CyberGrindPattern {
    fn index_mut(&mut self, range: RangeFull) -> &mut [Tile] {
        &mut self.tiles[range]
    }
}
