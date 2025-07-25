mod normal_fmt;
mod tile;
pub use tile::Prefab;
pub use tile::Tile;

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::ops::RangeFull;

pub mod error {
    use std::{
        error::Error,
        fmt::{Debug, Display},
        io,
    };

    /// Error type which is used
    /// in parsing methods.
    #[derive(Debug)]
    pub enum IoError {
        Io(io::Error),
        Parse(ParseError),
    }

    /// Error type which is returned when
    /// trying to parse invalid data
    #[derive(PartialEq, Clone)]
    pub struct ParseError {
        pub line: usize,
        pub column: usize,
        pub char: u8,
        pub kind: ParseErrorType,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ParseErrorType {
        /// Returns when a newline was expected,
        /// but got a different character.
        ExpectedNewline,
        /// Returns when parsed height is
        /// not between -50 and 50
        InvalidHeightValue,
        /// Returns when there are two negative
        /// signs in the parsed height
        DuplicateNegative,
        /// Returns if a number in parentheses
        /// starts with a zero
        LeadingZero,
        /// Returns when an unexpected character
        /// is found while parsing height
        InvalidHeightChar,
        /// Returns when an invalid prefab byte
        /// is found while parsing prefabs
        InvalidPrefab,
    }

    impl Error for IoError {}
    impl Error for ParseError {}
    impl Error for ParseErrorType {}

    impl Display for IoError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    impl Display for ParseErrorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Debug for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let kind_str = match self.kind {
                ParseErrorType::ExpectedNewline => "Expected newline but got",
                ParseErrorType::InvalidHeightChar => "Invalid height char",
                ParseErrorType::InvalidHeightValue => "Extreme height value:",
                ParseErrorType::LeadingZero => "Leading zero in parentheses",
                ParseErrorType::InvalidPrefab => "Invalid prefab character",
                ParseErrorType::DuplicateNegative => "Duplicate negative symbol",
            };

            match self.char {
                32..=126 | 161..=u8::MAX => write!(
                    f,
                    "Error parsing line {}, column {}: {} \"{}\"",
                    self.line,
                    self.column,
                    kind_str,
                    char::from(self.char)
                ),
                _ => write!(
                    f,
                    "Error parsing line {}, column {}: {} 0x{:02x}",
                    self.line, self.column, kind_str, self.char
                ),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::error::ParseError;

        #[test]
        fn valid_char_decode() {
            let mock_err_height = ParseError {
                line: 1,
                column: 3,
                char: b'-',
                kind: super::ParseErrorType::DuplicateNegative,
            };

            let dbg_str = format!("{mock_err_height:?}");
            assert_eq!(
                dbg_str,
                "Error parsing line 1, column 3: Duplicate negative symbol \"-\""
            );

            let mock_err_height = ParseError {
                line: 1,
                column: 3,
                char: b'g', // g for Gianni
                kind: super::ParseErrorType::InvalidPrefab,
            };

            let dbg_str = format!("{mock_err_height:?}");
            assert_eq!(
                dbg_str,
                "Error parsing line 1, column 3: Invalid prefab character \"g\""
            );
        }

        #[test]
        fn invalid_char_decode() {
            let mock_err_height = ParseError {
                line: 1,
                column: 3,
                char: 11,
                kind: super::ParseErrorType::ExpectedNewline,
            };

            let dbg_str = format!("{mock_err_height:?}");
            assert_eq!(
                dbg_str,
                "Error parsing line 1, column 3: Expected newline but got 0x0b"
            );

            let mock_err_height = ParseError {
                line: 1,
                column: 3,
                char: 11,
                kind: super::ParseErrorType::InvalidHeightChar,
            };

            let dbg_str = format!("{mock_err_height:?}");
            assert_eq!(
                dbg_str,
                "Error parsing line 1, column 3: Invalid height char 0x0b"
            );
        }
    }
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

pub struct CyberGrindPatternIter<'a> {
    tiles: &'a [Tile; 256],
    idx: usize,
}

impl<'a> IntoIterator for &'a CyberGrindPattern {
    type Item = Tile;
    type IntoIter = CyberGrindPatternIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CyberGrindPatternIter {
            tiles: &self.tiles,
            idx: 0,
        }
    }
}

impl<'a> Iterator for CyberGrindPatternIter<'a> {
    type Item = Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < 256 {
            let idx = self.idx;
            self.idx += 1;
            Some(self.tiles[idx])
        } else {
            None
        }
    }
}
