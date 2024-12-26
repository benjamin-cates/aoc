#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point(i64,i64);


impl Point {
    pub fn x(&self) -> i64{
        self.0
    }
    pub fn y(&self) -> i64 {
        self.1
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("({}, {})", self.0, self.1))
    }
}
impl<TR: Into<Point>> std::ops::Add<TR> for Point {
    type Output = Point;
    fn add(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}
impl<TR: Into<Point>> std::ops::Add<TR> for &Point {
    type Output = Point;
    fn add(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}
impl<T: Into<Point>> std::ops::AddAssign<T> for Point {
    fn add_assign(&mut self, rhs: T) {
        let point = rhs.into();
        self.0 += point.0;
        self.1 += point.1;
    }
}
impl<TR: Into<Point>> std::ops::Sub<TR> for Point {
    type Output = Point;
    fn sub(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 - rhs.0, lhs.1 - rhs.1)
    }
}
impl<TR: Into<Point>> std::ops::Sub<TR> for &Point {
    type Output = Point;
    fn sub(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 - rhs.0, lhs.1 - rhs.1)
    }
}
impl<T: Into<Point>> std::ops::SubAssign<T> for Point {
    fn sub_assign(&mut self, rhs: T) {
        let point = rhs.into();
        self.0 -= point.0;
        self.1 -= point.1;
    }
}
impl std::ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Mul<i64> for &Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::MulAssign<i64> for Point {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}
impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Point(value.0, value.1)
    }
}
impl From<Point> for (i64, i64) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}
impl From<&Point> for Point {
    fn from(value: &Point) -> Self {
        *value
    }
}
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point(value.0 as i64, value.1 as i64)
    }
}
#[derive(Hash, Clone, PartialEq, Eq)]
pub struct CharGrid {
    pub grid: Vec<Vec<char>>,
}
impl CharGrid {
    pub fn iter_range<T: Into<Point>, H: Into<Point>>(top_left: T, bottom_right: H) -> impl Iterator<Item=Point> {
        let top_left = top_left.into();
        let bottom_right = bottom_right.into();
        let mut x = top_left.0;
        let mut y = top_left.1;
        iter::from_fn(move || loop {
            x += 1;
            if x == bottom_right.0 {
                x = top_left.0;
                y += 1;
            }
            if y > bottom_right.1 {
                return None;
            }
            return Some(Point(x, y));
        })
    }
    pub fn new_fill(width: usize, height: usize, ch: char) -> CharGrid {
        CharGrid {
            grid: vec![vec![ch; width]; height],
        }
    }
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
    pub fn width(&self) -> i64 {
        self.grid[0].len() as i64
    }
    pub fn height(&self) -> i64 {
        self.grid.len() as i64
    }
    pub fn find(&self, ch: char) -> Option<Point> {
        for y in 0..self.height() as usize {
            for x in 0..self.width() as usize {
                if self.grid[y][x] == ch {
                    return Some(Point(x as i64, y as i64));
                }
            }
        }
        None
    }
    pub fn find_all<'a>(&'a self, ch: char) -> impl Iterator<Item = Point> + use<'a> {
        let mut x = 0;
        let mut y = 0;
        let width = self.width() as usize;
        let height = self.height() as usize;
        iter::from_fn(move || loop {
            if self.grid[y][x] == ch {
                let out = Some(Point(x as i64, y as i64));
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
        })
    }
    pub fn iter_points<'a>(&'a self) -> impl Iterator<Item = (Point, char)> + use<'a> {
        let mut x = 0;
        let mut y = 0;
        let width = self.width() as usize;
        let height = self.height() as usize;
        iter::from_fn(move || {
            let out = Some((Point(x as i64, y as i64), self.grid[x][y]));
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
        self.get_ref(pos).copied().unwrap()
    }
    pub fn is_inside<T: Into<Point>>(&self, pos: T) -> bool {
        self.get_ref(pos).is_some()
    }
    pub fn get<T: Into<Point>>(&self, pos: T) -> Option<char> {
        self.get_ref(pos).copied()
    }
    fn get_ref<T: Into<Point>>(&self, pos: T) -> Option<&char> {
        let point: Point = pos.into();
        if point.0 < 0 || point.1 < 0 || point.0 >= self.width() || point.1 >= self.height() {
            return None;
        }
        Some(&self.grid[point.1 as usize][point.0 as usize])
    }
    fn get_ref_mut<T: Into<Point>>(&mut self, pos: T) -> Option<&mut char> {
        let point: Point = pos.into();
        if point.0 < 0 || point.1 < 0 || point.0 >= self.width() || point.1 >= self.height() {
            return None;
        }
        Some(&mut self.grid[point.1 as usize][point.0 as usize])
    }
    pub fn set<T: Into<Point>>(&mut self, pos: T, value: char) -> Option<()> {
        *self.get_ref_mut(pos)? = value;
        Some(())
    }
}

impl From<&str> for CharGrid {
    fn from(value: &str) -> Self {
        CharGrid::new(value)
    }
}

impl std::fmt::Display for CharGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in self.grid.iter() {
            for char in line.iter() {
                f.write_char(*char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
impl<T: Into<Point>> std::ops::Index<T> for CharGrid {
    type Output = char;
    fn index(&self, index: T) -> &Self::Output {
        self.get_ref(index).unwrap()
    }
}
impl<T: Into<Point>> std::ops::IndexMut<T> for CharGrid {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        self.get_ref_mut(index).unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

use std::{
    fmt::{Display, Formatter, Write},
    iter,
};

use Direction::*;

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            North => (0i64, -1i64),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
        .into()
    }
}
impl From<&Direction> for Point {
    fn from(value: &Direction) -> Self {
        match *value {
            North => (0i64, -1i64),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
        .into()
    }
}

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
        (*self).into()
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
