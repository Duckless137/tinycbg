use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{CyberGrindPattern, Tile};

pub struct Iter<'a> {
    pat: &'a [Tile; 256],
    idx: usize,
}

#[derive(Clone, Copy)]
pub struct Row<'a> {
    tiles: &'a [Tile],
}

pub struct RowIter<'a> {
    tiles: &'a [Tile],
    idx: usize,
}

impl<'a> IntoIterator for &'a CyberGrindPattern {
    type Item = Row<'a>;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            pat: &self.tiles,
            idx: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Row<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 16 {
            return None;
        }

        let index = self.idx;
        self.idx += 1;
        let row = &self.pat[index * 16..(index + 1) * 16];
        Some(Row { tiles: row })
    }
}

impl<'a> IntoIterator for Row<'a> {
    type IntoIter = RowIter<'a>;
    type Item = Tile;
    fn into_iter(self) -> Self::IntoIter {
        RowIter {
            tiles: self.tiles,
            idx: 0,
        }
    }
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 16 {
            return None;
        }
        let index = self.idx;
        self.idx += 1;

        Some(self.tiles[index])
    }
}

pub struct IterMut<'a> {
    pat: &'a mut [Tile; 256],
    idx: usize,
}

#[derive(Clone, Copy)]
pub struct RowMut<'a> {
    tiles: *mut [Tile],
    phantom_data: PhantomData<&'a i32>,
}

pub struct RowIterMut<'a> {
    tiles: *mut [Tile],
    idx: usize,
    phantom_data: PhantomData<&'a i32>,
}

impl<'a> IntoIterator for &'a mut CyberGrindPattern {
    type Item = RowMut<'a>;
    type IntoIter = IterMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            pat: &mut self.tiles,
            idx: 0,
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = RowMut<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 16 {
            return None;
        }
        let index = self.idx;
        self.idx += 1;
        let row = &raw mut self.pat[index * 16..(index + 1) * 16];

        Some(RowMut {
            tiles: row,
            phantom_data: PhantomData,
        })
    }
}

impl<'a> IntoIterator for RowMut<'a> {
    type Item = &'a mut Tile;
    type IntoIter = RowIterMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        RowIterMut {
            tiles: self.tiles,
            idx: 0,
            phantom_data: PhantomData,
        }
    }
}

impl<'a> Iterator for RowIterMut<'a> {
    type Item = &'a mut Tile;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 16 {
            return None;
        }

        let index = self.idx;
        self.idx += 1;

        let res = unsafe { &mut (*self.tiles)[index] };
        Some(res)
    }
}

impl<'a> Deref for Row<'a> {
    type Target = [Tile];
    fn deref(&self) -> &Self::Target {
        self.tiles
    }
}

impl<'a> Deref for RowMut<'a> {
    type Target = [Tile];
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.tiles }
    }
}

impl<'a> DerefMut for RowMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.tiles }
    }
}

impl<'a> Debug for Row<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a> Debug for RowMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
