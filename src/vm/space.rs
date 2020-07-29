//! Funge-Space 2D Toroidal Grid Representation.

#[derive(Debug, Clone)]
pub struct FungeSpace {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<i64>>,
}

impl FungeSpace {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![32; width]; height],
        }
    }
}
