use std::boxed::Box;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use tpscube_core::Move;

pub struct Progress {
    filled: usize,
    total: usize,
}

impl Progress {
    pub fn complete(&self) -> bool {
        self.filled == self.total
    }
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} / {}", self.filled, self.total)
    }
}

pub struct MoveTable<const N: usize, const M: usize> {
    contents: Box<[[Option<u16>; M]; N]>,
}

pub struct PruneTable1D<const N: usize> {
    contents: Box<[Option<u8>; N]>,
}

pub struct PruneTable2D<const A: usize, const B: usize> {
    contents: Box<[[Option<u8>; B]; A]>,
}

impl<const N: usize, const M: usize> MoveTable<N, M> {
    pub fn get(&self, idx: u16, mv: Move) -> Option<u16> {
        self.contents[idx as usize][mv as usize]
    }

    pub fn set(&mut self, idx: u16, mv: Move, value: u16) {
        self.contents[idx as usize][mv as usize] = Some(value);
    }

    pub fn update(&mut self, old: u16, mv: Move, new: u16) -> bool {
        if self.get(old, mv).is_none() {
            self.set(old, mv, new);
            true
        } else {
            assert_eq!(self.get(old, mv), Some(new));
            false
        }
    }

    pub fn progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.contents.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    pub fn progress_filtered<F: Fn(Move) -> bool>(&self, f: F) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.contents.as_ref() {
            for (j_idx, j) in i.iter().enumerate() {
                if f(Move::try_from(j_idx as u8).unwrap()) {
                    total += 1;
                    if j.is_some() {
                        filled += 1;
                    }
                }
            }
        }
        Progress { filled, total }
    }

    pub fn write(&self, name: &str) {
        let mut out = BufWriter::new(File::create(Path::new(name)).unwrap());
        for i in self.as_ref() {
            for j in i {
                out.write(&(j.unwrap_or(0xffff) as u16).to_le_bytes())
                    .unwrap();
            }
        }
    }
}

impl<const N: usize, const M: usize> AsRef<[[Option<u16>; M]; N]> for MoveTable<N, M> {
    fn as_ref(&self) -> &[[Option<u16>; M]; N] {
        self.contents.as_ref()
    }
}

impl<const N: usize, const M: usize> Default for MoveTable<N, M> {
    fn default() -> Self {
        Self {
            contents: box [[None; M]; N],
        }
    }
}

impl<const N: usize> PruneTable1D<N> {
    pub fn get(&self, idx: u16) -> Option<u8> {
        self.contents[idx as usize]
    }

    pub fn set(&mut self, idx: u16, value: u8) {
        self.contents[idx as usize] = Some(value);
    }

    pub fn update(&mut self, old: u16, new: u16) -> bool {
        let value = self.get(old).unwrap() + 1;
        if self.get(new).is_none() || Some(value) < self.get(new) {
            self.set(new, value);
            true
        } else {
            false
        }
    }

    pub fn update_as_solution(&mut self, new: u16) -> bool {
        if self.get(new).is_none() || Some(0) < self.get(new) {
            self.set(new, 0);
            true
        } else {
            false
        }
    }

    pub fn progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.contents.as_ref() {
            total += 1;
            if i.is_some() {
                filled += 1;
            }
        }
        Progress { filled, total }
    }

    pub fn write(&self, name: &str) {
        let mut out = BufWriter::new(File::create(Path::new(name)).unwrap());
        for i in self.as_ref() {
            out.write(&[i.unwrap_or(0xff) as u8]).unwrap();
        }
    }
}

impl<const N: usize> AsRef<[Option<u8>; N]> for PruneTable1D<N> {
    fn as_ref(&self) -> &[Option<u8>; N] {
        self.contents.as_ref()
    }
}

impl<const N: usize> Default for PruneTable1D<N> {
    fn default() -> Self {
        Self {
            contents: box [None; N],
        }
    }
}

impl<const A: usize, const B: usize> PruneTable2D<A, B> {
    pub fn get(&self, a: u16, b: u16) -> Option<u8> {
        self.contents[a as usize][b as usize]
    }

    pub fn set(&mut self, a: u16, b: u16, value: u8) {
        self.contents[a as usize][b as usize] = Some(value);
    }

    pub fn update(&mut self, old_a: u16, old_b: u16, new_a: u16, new_b: u16) -> bool {
        let value = self.get(old_a, old_b).unwrap() + 1;
        if self.get(new_a, new_b).is_none() || Some(value) < self.get(new_a, new_b) {
            self.set(new_a, new_b, value);
            true
        } else {
            false
        }
    }

    pub fn update_as_solution(&mut self, new_a: u16, new_b: u16) -> bool {
        if self.get(new_a, new_b).is_none() || Some(0) < self.get(new_a, new_b) {
            self.set(new_a, new_b, 0);
            true
        } else {
            false
        }
    }

    pub fn progress(&self) -> Progress {
        let mut filled = 0;
        let mut total = 0;
        for i in self.contents.as_ref() {
            for j in i {
                total += 1;
                if j.is_some() {
                    filled += 1;
                }
            }
        }
        Progress { filled, total }
    }

    pub fn write(&self, name: &str) {
        let mut out = BufWriter::new(File::create(Path::new(name)).unwrap());
        for i in self.as_ref() {
            for j in i {
                out.write(&[j.unwrap_or(0xff) as u8]).unwrap();
            }
        }
    }

    pub fn write_min(&self, name: &str) {
        let mut out = BufWriter::new(File::create(Path::new(name)).unwrap());
        for i in self.as_ref() {
            let mut min = i[0].unwrap();
            for j in i {
                min = std::cmp::min(min, j.unwrap());
            }
            out.write(&[min as u8]).unwrap();
        }
    }
}

impl<const A: usize, const B: usize> AsRef<[[Option<u8>; B]; A]> for PruneTable2D<A, B> {
    fn as_ref(&self) -> &[[Option<u8>; B]; A] {
        self.contents.as_ref()
    }
}

impl<const A: usize, const B: usize> Default for PruneTable2D<A, B> {
    fn default() -> Self {
        Self {
            contents: box [[None; B]; A],
        }
    }
}
