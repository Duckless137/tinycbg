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
#[derive(PartialEq, Clone, Debug)]
pub struct ParseError {
    pub line: u32,
    pub column: u32,
    pub kind: ParseErrorType,
    pub char: u8,
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

impl From<io::Error> for IoError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<ParseError> for IoError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl Error for IoError {}
impl Error for ParseError {}
impl Error for ParseErrorType {}

impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Display for ParseError {
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
impl Display for ParseErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
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

        let dbg_str = format!("{mock_err_height}");
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

        let dbg_str = format!("{mock_err_height}");
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

        let dbg_str = format!("{mock_err_height}");
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

        let dbg_str = format!("{mock_err_height}");
        assert_eq!(
            dbg_str,
            "Error parsing line 1, column 3: Invalid height char 0x0b"
        );
    }
}
