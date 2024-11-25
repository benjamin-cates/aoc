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
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}
pub struct CharGrid {
    pub grid: Vec<Vec<char>>,
}
impl CharGrid {
    pub fn new(chars: &str) -> CharGrid {
        let out = CharGrid {
            grid: chars.lines().map(|x| x.chars().collect()).collect(),
        };
        let width = out.grid[0].len();
        for row in out.grid.iter() {
            if row.len() != width {
                panic!("Mismatching widths! First row had width {} and this row had width {}.", width, row.len());
            }
        }
        out
    }
    pub fn width(&self) -> i32 {
        self.grid[0].len() as i32
    }
    pub fn height(&self) -> i32 {
        self.grid.len() as i32
    }
    pub fn get<T>(&self, pos: T) -> Option<char> 
        where T: Into<Point>
    {
        let point: Point = pos.into();
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            return None;
        }
        Some(self.grid[point.y as usize][point.x as usize])
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
    pub fn deflect_slash(&self) -> Direction {
        match self {
            North => East,
            South => West,
            West => South,
            East => North,
        }
    }
    pub fn deflect_backslash(&self) -> Direction {
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
impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(East),
            'D' => Ok(South),
            'L' => Ok(West),
            'U' => Ok(North),
            'r' => Ok(East),
            'd' => Ok(South),
            'l' => Ok(West),
            'u' => Ok(North),
            'v' => Ok(South),
            '^' => Ok(North),
            '>' => Ok(East),
            '<' => Ok(West),
            _ => Err(()),
        }
    }
}
