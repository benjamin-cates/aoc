#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}
pub struct CharGrid {
    pub grid: Vec<Vec<char>>,
}
impl CharGrid {
    pub fn new(chars: &str) -> CharGrid {
        CharGrid {
            grid: chars.lines().map(|x| x.chars().collect()).collect(),
        }
    }
    pub fn width(&self) -> i32 {
        self.grid[0].len() as i32
    }
    pub fn height(&self) -> i32 {
        self.grid.len() as i32
    }
    pub fn get(&self, point: Point) -> Option<char> {
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            return None;
        }
        Some(self.grid[point.y as usize][point.x as usize])
    }
}

pub enum Dir {
    North,
    South,
    East,
    West,
}

use Dir::*;

impl Dir {
    pub fn opposite(&self) -> Dir {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
    pub fn deflect_slash(&self) -> Dir {
        match self {
            North => East,
            South => West,
            West => South,
            East => North,
        }
    }
    pub fn deflect_backslash(&self) -> Dir {
        match self {
            North => West,
            South => East,
            East => South,
            West => North,
        }
    }
    pub fn step(&self) -> Point {
        match self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
        .into()
    }
}
