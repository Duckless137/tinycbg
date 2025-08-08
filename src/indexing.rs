use crate::{CyberGrindPattern, Tile};
use core::panic;
use std::ops::{Bound, Index, IndexMut, Range, RangeBounds, RangeFrom, RangeFull, RangeTo};

macro_rules! impl_index {
    ($idx:ty, $out:ty) => {
        impl Index<$idx> for CyberGrindPattern {
            type Output = $out;
            fn index(&self, index: $idx) -> &Self::Output {
                &self.tiles[index]
            }
        }
        impl IndexMut<$idx> for CyberGrindPattern {
            fn index_mut(&mut self, index: $idx) -> &mut $out {
                &mut self.tiles[index]
            }
        }
    };
}

impl_index!(usize, Tile);
impl_index!(Range<usize>, [Tile]);
impl_index!(RangeTo<usize>, [Tile]);
impl_index!(RangeFrom<usize>, [Tile]);
impl_index!(RangeFull, [Tile]);

type Point = (usize, usize);

impl Index<Point> for CyberGrindPattern {
    type Output = Tile;
    fn index(&self, coordinate: (usize, usize)) -> &Self::Output {
        let index = coordinate.1 * 16 + coordinate.0;
        &self.tiles[index]
    }
}

impl IndexMut<Point> for CyberGrindPattern {
    fn index_mut(&mut self, coordinate: (usize, usize)) -> &mut Tile {
        let index = coordinate.1 * 16 + coordinate.0;
        &mut self.tiles[index]
    }
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    let mut c;

    while b != 0 {
        c = a % b;
        a = b;
        b = c;
    }

    a
}

fn draw_line(point_a: Point, point_b: Point, buf: &mut [u8; 16]) -> usize {
    let dx = point_b.0 - point_a.0;
    let dy = point_b.1 - point_a.1;

    let gcd = greatest_common_divisor(dx, dy);

    let step_x = dx / gcd;
    let step_y = dy / gcd;

    for (i, item) in buf.iter_mut().enumerate().take(gcd + 1) {
        let x_pos = point_a.0 + i * step_x;
        let y_pos = point_a.1 + i * step_y;

        let idx = x_pos + y_pos * 16;
        *item = idx as u8;
    }

    gcd + 1
}

impl CyberGrindPattern {
    /// Draws a line between two points and
    /// gets a mutable reference to all
    /// points on that line.
    pub fn line<'a>(&'a mut self, index: Range<Point>) -> Line<'a> {
        let point_a = match index.start_bound() {
            Bound::Unbounded => panic!("Please provide a lower bound"),
            Bound::Included(point) => *point,
            Bound::Excluded(_) => panic!("Please only provide an included lower bound"),
        };
        let point_b = match index.end_bound() {
            Bound::Unbounded => panic!("Please provide an upper bound"),
            Bound::Included(_) => panic!("Please only provide an excluded lower bound"),
            Bound::Excluded(point) => *point,
        };

        assert!(point_a.0 < 16, "X value of lower bound is out of range!");
        assert!(point_a.1 < 16, "Y value of lower bound is out of range!");

        assert!(point_b.0 < 16, "X value of upper bound is out of range!");
        assert!(point_b.1 < 16, "Y value of upper bound is out of range!");

        let mut line = Line {
            data: self,
            buf: [0; 16],
            len: 0,
            idx: 0,
        };
        line.len = draw_line(point_a, point_b, &mut line.buf) as u8;

        line
    }
}

pub struct Line<'a> {
    data: &'a mut CyberGrindPattern,
    // In order to keep things compact,
    // the buf stores an array of indexes,
    // rather than direct pointers to the
    // tiles.
    buf: [u8; 16],
    len: u8,
    // By using u8 we can pack in
    // another value here to use
    // this as an iterator, without
    // increasing the size of the
    // struct.
    idx: u8,
}

impl<'a> Line<'a> {
    // Sets the entire line
    // to one tile
    pub fn set(&mut self, tile: Tile) {
        for item in self {
            *item = tile;
        }
    }

    // Returns the length of the line
    pub fn len(&self) -> usize {
        self.len as usize
    }
}

impl<'a> Iterator for Line<'a> {
    type Item = &'a mut Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            return None;
        }

        let idx = self.buf[self.idx as usize] as usize;
        self.idx += 1;
        let ptr = self.data as *mut CyberGrindPattern;
        // Oh my god why
        unsafe { Some(&mut *(ptr as *mut Tile).add(idx)) }
    }
}

impl<'a> Index<usize> for Line<'a> {
    type Output = Tile;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 256, "Index is out of bounds");
        let idx = self.buf[self.idx as usize] as usize;
        &self.data[idx]
    }
}

impl<'a> IndexMut<usize> for Line<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Tile {
        assert!(index < 256, "Index is out of bounds");
        let idx = self.buf[self.idx as usize] as usize;
        &mut self.data[idx]
    }
}
