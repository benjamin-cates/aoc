#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),std::fmt::Error> {
        f.write_fmt(format_args!("({}, {})",self.x,self.y))
    }
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
impl<T: Into<Point>> std::ops::AddAssign<T> for Point {
    fn add_assign(&mut self, rhs: T) {
        let point = rhs.into();
        self.x += point.x;
        self.y += point.y;
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T: Into<Point>> std::ops::SubAssign<T> for Point {
    fn sub_assign(&mut self, rhs: T) {
        let point = rhs.into();
        self.x -= point.x;
        self.y -= point.y;
    }
}
impl std::ops::Mul<Point> for i32 {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: rhs.x * self,
            y: rhs.y * self,
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
#[derive(Hash, Clone, PartialEq, Eq)]
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
                panic!(
                    "Mismatching widths! First row had width {} and this row had width {}.",
                    width,
                    row.len()
                );
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
    pub fn find(&self, ch: char) -> Option<Point> {
        for y in 0..self.height() as usize {
            for x in 0..self.width() as usize {
                if self.grid[y][x] == ch {
                    return Some((x,y).into());
                }
            }
        }
        None
    }
    pub fn find_all<'a>(&'a self, ch: char) -> impl Iterator<Item=Point> + use<'a> {
        let mut x = 0;
        let mut y = 0;
        let width = self.width() as usize;
        let height = self.height() as usize;
        iter::from_fn(move || {
            loop {
                if self.grid[y][x] == ch {
                    let out = Some((x,y).into());
                    x += 1;
                    if x == width {
                        x = 0;
                        y += 1;
                    }
                    return out;
                }
                x += 1;
                if x == width {
                    x = 0;
                    y += 1;
                }
                if y >= height {
                    return None;
                }
            }
        })
    }
    pub fn iter_points<'a>(&'a self) -> impl Iterator<Item=(Point,char)> + use<'a> {
        let mut x = 0;
        let mut y = 0;
        let width = self.width() as usize;
        let height = self.height() as usize;
        iter::from_fn(move || {
            let out = Some(((x,y).into(),self.grid[x][y]));
            x += 1;
            if x == width {
                x = 0;
                y += 1;
            }
            if y >= height {
                return None;
            }
            out
        })

    }
    pub fn get_unwrap<T: Into<Point>>(&self, pos: T) -> char {
        let point: Point = pos.into();
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            panic!("Tried to get point {} outside of char grid",point);
        }
        self.grid[point.y as usize][point.x as usize]

    }
    pub fn is_inside<T: Into<Point>>(&self, pos: T) -> bool {
        let point: Point = pos.into();
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            false
        }
        else {
            true
        }
    }
    pub fn get<T>(&self, pos: T) -> Option<char>
    where
        T: Into<Point>,
    {
        let point: Point = pos.into();
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            return None;
        }
        Some(self.grid[point.y as usize][point.x as usize])
    }
    pub fn set<T: Into<Point>>(&mut self, pos: T, value: char) -> Option<()> {
        let point: Point = pos.into();
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            return None;
        }
        self.grid[point.y as usize][point.x as usize] = value;
        Some(())
    }
}

impl std::fmt::Display for CharGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),std::fmt::Error> {
        for line in self.grid.iter() {
            for char in line.iter() {
                f.write_char(*char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
impl<T> std::ops::Index<T> for CharGrid where T: Into<Point> {
    type Output = char;
    fn index(&self, index: T) -> &Self::Output {
        let point = index.into();
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            panic!("Tried to get point {} outside of char grid",point);
        }
        &self.grid[point.y as usize][point.x as usize]
    }
}
impl<T> std::ops::IndexMut<T> for CharGrid where T: Into<Point> {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let point = index.into();
        if point.x < 0 || point.y < 0 || point.x >= self.width() || point.y >= self.height() {
            panic!("Tried to get point {} outside of char grid",point);
        }
        &mut self.grid[point.y as usize][point.x as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

use std::{fmt::{Display, Formatter, Write}, iter};

use Direction::*;

impl Direction {
    pub fn iter_all() -> impl Iterator<Item = Direction> {
        [North, South, East, West].into_iter()
    }
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

    pub fn rotate_right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    pub fn rotate_left(&self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
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
