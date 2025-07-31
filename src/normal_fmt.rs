use crate::{
    error::{IoError, ParseError, ParseErrorType},
    CyberGrindPattern, Prefab,
};
use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    path::Path,
};

const MAX_FILE_SIZE: usize = 1569;

impl CyberGrindPattern {
    /// Creates a new file at path `path`. If one already exists,
    /// it is truncated. Outputs a Cybergrind Pattern File to that
    /// path.
    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        self.write(&mut file)
    }

    /// Takes in a file and writes a Cybergrind Pattern to it.
    pub fn write(&self, file: &mut File) -> Result<(), io::Error> {
        let mut buf = Box::new([0; MAX_FILE_SIZE]);
        let mut buf_idx = 0;

        let mut tile_idx = 0;
        for _row in 0..16 {
            for _column in 0..16 {
                let tile = self.tiles[tile_idx];
                let height = tile.height();

                // Actually sobbing right now
                if (0..9).contains(&height) {
                    buf[buf_idx] = (height + 48) as u8;
                    buf_idx += 1;
                } else {
                    let dear_god_why = format!("({height})");
                    for byte in dear_god_why.as_bytes() {
                        buf[buf_idx] = *byte;
                        buf_idx += 1;
                    }
                }

                tile_idx += 1;
            }
            buf[buf_idx] = b'\n';
            buf_idx += 1;
        }

        buf[buf_idx] = b'\n';
        buf_idx += 1;

        tile_idx = 0;
        for _row in 0..16 {
            for _column in 0..16 {
                let tile = self.tiles[tile_idx];
                let prefab = tile.prefab();

                let char = match prefab {
                    Prefab::None => b'0',
                    Prefab::Melee => b'n',
                    Prefab::Projectile => b'p',
                    Prefab::HideousMass => b'H',
                    Prefab::Stairs => b's',
                    Prefab::JumpPad => b'J',
                };

                buf[buf_idx] = char;
                buf_idx += 1;

                tile_idx += 1;
            }
            buf[buf_idx] = b'\n';
            buf_idx += 1;
        }

        let mut writer = BufWriter::new(file);

        writer.write_all(&buf[..buf_idx])
    }

    fn check_for_newline(line: u32, column: u32, byte: u8) -> Result<(), ParseError> {
        if byte != b'\n' {
            Err(ParseError {
                line,
                column,
                char: byte,
                kind: ParseErrorType::ExpectedNewline,
            })
        } else {
            Ok(())
        }
    }

    // Returns:
    // - height
    // - new column idx
    // - increased buf_idx
    fn parse_parentheses(
        bytes: &[u8],
        buf_idx: usize,
        line: u32,
        column: u32,
    ) -> Result<(i8, u32, usize), ParseError> {
        let mut is_negative = false;
        let mut height: i8 = 0;
        let mut buf_idx = buf_idx;

        let mut column = column + 1;
        buf_idx += 1;
        let mut char = bytes[buf_idx];

        while char != b')' {
            if char == b'-' {
                if is_negative {
                    return Err(ParseError {
                        line,
                        column,
                        char,
                        kind: ParseErrorType::DuplicateNegative,
                    });
                } else {
                    is_negative = true;
                }
            } else {
                if !(48..=57).contains(&char) {
                    return Err(ParseError {
                        line,
                        column,
                        char,
                        kind: ParseErrorType::InvalidHeightChar,
                    });
                }

                if char == 48 && height == 0 {
                    return Err(ParseError {
                        line,
                        column,
                        char,
                        kind: ParseErrorType::LeadingZero,
                    });
                }

                height *= 10;
                height += char as i8 - 48;
            }

            column += 1;
            buf_idx += 1;
            char = bytes[buf_idx];
        }

        if is_negative {
            height *= -1;
        }

        if !(-50..=50).contains(&height) {
            return Err(ParseError {
                line,
                column,
                char,
                kind: ParseErrorType::InvalidHeightValue,
            });
        }

        Ok((height, column, buf_idx))
    }

    /// Takes in a series of bytes and tries
    /// to turn them into a Cybergrind Pattern.
    pub fn parse(bytes: &[u8]) -> Result<CyberGrindPattern, ParseError> {
        let mut pattern = CyberGrindPattern::new();

        let mut pat_idx = 0;
        let mut buf_idx = 0;
        let mut line = 1;
        let mut char;

        for _row in 0..16 {
            let mut column = 1;
            for _column in 0..16 {
                char = bytes[buf_idx];
                if char == b'(' {
                    let height;
                    (height, column, buf_idx) =
                        Self::parse_parentheses(bytes, buf_idx, line, column)?;

                    pattern[pat_idx].set_height(height);
                } else {
                    if !(48..=57).contains(&char) {
                        return Err(ParseError {
                            line,
                            column,
                            char,
                            kind: ParseErrorType::InvalidHeightChar,
                        });
                    }
                    pattern[pat_idx].set_height(char as i8 - 48);
                }

                column += 1;
                pat_idx += 1;
                buf_idx += 1;
            }

            Self::check_for_newline(line, column, bytes[buf_idx])?;
            buf_idx += 1;

            line += 1;
        }

        Self::check_for_newline(line, 1, bytes[buf_idx])?;

        buf_idx += 1;
        line += 1;

        pat_idx = 0;

        for _row in 0..16 {
            for column in 1..17 {
                char = bytes[buf_idx];

                let prefab = match Prefab::try_from(char) {
                    Ok(prefab) => prefab,
                    Err(kind) => {
                        return Err(ParseError {
                            line,
                            column,
                            char,
                            kind,
                        })
                    }
                };

                pattern[pat_idx].set_prefab(prefab);
                pat_idx += 1;
                buf_idx += 1;
            }

            Self::check_for_newline(line, 17, bytes[buf_idx])?;
            buf_idx += 1;

            line += 1;
        }

        Ok(pattern)
    }

    /// Takes in a string and tries to turn it into
    /// a Cybergrind pattern.
    pub fn parse_str(string: &str) -> Result<CyberGrindPattern, ParseError> {
        Self::parse(string.as_bytes())
    }

    /// Takes in a string and tries to read
    /// it as a Cybergrind pattern.
    pub fn parse_file(file: &mut File) -> Result<CyberGrindPattern, IoError> {
        let mut buf = Box::new([0; MAX_FILE_SIZE]);
        let mut reader = BufReader::new(file);
        let bytes_read = match reader.read(buf.as_mut()) {
            Ok(bytes_read) => bytes_read,
            Err(err) => return Err(IoError::Io(err)),
        };
        match Self::parse(&buf[..bytes_read]) {
            Ok(pat) => Ok(pat),
            Err(e) => Err(IoError::Parse(e)),
        }
    }

    /// Tries to open a file at path `path` and reads
    /// it as a Cybergrind Patter.
    pub fn parse_path<P: AsRef<Path>>(path: P) -> Result<CyberGrindPattern, IoError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(IoError::Io(err)),
        };

        Self::parse_file(&mut file)
    }
}
