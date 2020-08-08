//! Funge-Space 2D Toroidal Grid and Spatial Memory Architecture.
//!
//! Funge-Space implements a two-dimensional discrete coordinate grid where
//! instructions, state matrices, and memory cells reside in the same address space.
//! The grid supports toroidal wrapping across configurable bounds and tracks
//! execution heatmaps for cycle-accurate profiling.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use std::collections::HashMap;

/// Represents a 2D discrete coordinate in Funge-Space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn offset(self, dx: i64, dy: i64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

/// Bounded 2D toroidal Funge-Space with sparse overlay for extended coordinates.
#[derive(Debug, Clone)]
pub struct FungeSpace {
    pub width: usize,
    pub height: usize,
    /// Fast primary dense grid for execution bounding box.
    dense: Vec<i64>,
    /// Sparse hash table for cells outside standard dense bounding box.
    sparse: HashMap<(i64, i64), i64>,
    /// Execution heatmap tracking instruction access frequency.
    pub heatmap: Vec<u64>,
    /// Total cell write count for memory mutation audits.
    pub mutation_count: u64,
}

impl FungeSpace {
    /// Creates a new Funge-Space initialized with ASCII space characters (value 32).
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            dense: vec![32; size],
            sparse: HashMap::new(),
            heatmap: vec![0; size],
            mutation_count: 0,
        }
    }

    /// Loads Befunge source code string into Funge-Space at the given origin (x0, y0).
    pub fn load_source(&mut self, source: &str, x0: i64, y0: i64) {
        for (row_idx, line) in source.lines().enumerate() {
            let y = y0 + row_idx as i64;
            for (col_idx, ch) in line.chars().enumerate() {
                let x = x0 + col_idx as i64;
                self.put(x, y, ch as i64);
            }
        }
    }

    /// Reads value from Funge-Space at discrete coordinate (x, y).
    #[inline]
    pub fn get(&self, x: i64, y: i64) -> i64 {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            let idx = (y as usize) * self.width + (x as usize);
            self.dense[idx]
        } else {
            *self.sparse.get(&(x, y)).unwrap_or(&32)
        }
    }

    /// Writes value into Funge-Space at discrete coordinate (x, y).
    #[inline]
    pub fn put(&mut self, x: i64, y: i64, val: i64) {
        self.mutation_count += 1;
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            let idx = (y as usize) * self.width + (x as usize);
            self.dense[idx] = val;
        } else {
            self.sparse.insert((x, y), val);
        }
    }

    /// Increments the access heatmap counter for execution profiling.
    #[inline]
    pub fn record_exec(&mut self, x: i64, y: i64) {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            let idx = (y as usize) * self.width + (x as usize);
            self.heatmap[idx] = self.heatmap[idx].saturating_add(1);
        }
    }

    /// Normalizes coordinate within the toroidal primary boundaries.
    #[inline]
    pub fn wrap_coord(&self, x: i64, y: i64) -> (i64, i64) {
        let w = self.width as i64;
        let h = self.height as i64;
        let wx = ((x % w) + w) % w;
        let wy = ((y % h) + h) % h;
        (wx, wy)
    }

    /// Renders a rectangular sub-region of Funge-Space as a printable ASCII string.
    pub fn dump_region(&self, x0: i64, y0: i64, x1: i64, y1: i64) -> String {
        let mut out = String::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                let v = self.get(x, y);
                if (32..=126).contains(&v) {
                    out.push((v as u8) as char);
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }
        out
    }

    /// Computes the bounding box of non-space cells currently stored in the grid.
    pub fn bounding_box(&self) -> (i64, i64, i64, i64) {
        let mut min_x = self.width as i64;
        let mut min_y = self.height as i64;
        let mut max_x = 0i64;
        let mut max_y = 0i64;
        let mut found = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if self.dense[idx] != 32 {
                    min_x = min_x.min(x as i64);
                    min_y = min_y.min(y as i64);
                    max_x = max_x.max(x as i64);
                    max_y = max_y.max(y as i64);
                    found = true;
                }
            }
        }

        for &(sx, sy) in self.sparse.keys() {
            if self.sparse.get(&(sx, sy)) != Some(&32) {
                min_x = min_x.min(sx);
                min_y = min_y.min(sy);
                max_x = max_x.max(sx);
                max_y = max_y.max(sy);
                found = true;
            }
        }

        if !found {
            (0, 0, 0, 0)
        } else {
            (min_x, min_y, max_x, max_y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funge_space_initialization() {
        let space = FungeSpace::new(80, 25);
        assert_eq!(space.get(0, 0), 32);
        assert_eq!(space.get(79, 24), 32);
        assert_eq!(space.get(100, 200), 32);
    }

    #[test]
    fn test_funge_space_put_and_get() {
        let mut space = FungeSpace::new(80, 25);
        space.put(10, 5, 42);
        assert_eq!(space.get(10, 5), 42);
        assert_eq!(space.mutation_count, 1);

        // Sparse cell outside primary grid
        space.put(500, 600, 99);
        assert_eq!(space.get(500, 600), 99);
        assert_eq!(space.mutation_count, 2);
    }

    #[test]
    fn test_funge_space_toroidal_wrap() {
        let space = FungeSpace::new(80, 25);
        assert_eq!(space.wrap_coord(80, 0), (0, 0));
        assert_eq!(space.wrap_coord(-1, 0), (79, 0));
        assert_eq!(space.wrap_coord(0, 25), (0, 0));
        assert_eq!(space.wrap_coord(0, -1), (0, 24));
    }

    #[test]
    fn test_funge_space_load_source() {
        let mut space = FungeSpace::new(80, 25);
        let src = ">25*.\n@";
        space.load_source(src, 0, 0);
        assert_eq!(space.get(0, 0), '>' as i64);
        assert_eq!(space.get(1, 0), '2' as i64);
        assert_eq!(space.get(2, 0), '5' as i64);
        assert_eq!(space.get(3, 0), '*' as i64);
        assert_eq!(space.get(4, 0), '.' as i64);
        assert_eq!(space.get(0, 1), '@' as i64);
    }
}
