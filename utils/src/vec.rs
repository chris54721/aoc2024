use std::ops::{Add, AddAssign, Index, Mul, Div, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vec2(pub isize, pub isize);

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from((x, y): (isize, isize)) -> Self {
        Vec2(x, y)
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Vec2(x as isize, y as isize)
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Vec2(x as isize, y as isize)
    }
}

impl Index<usize> for Vec2 {
    type Output = isize;
    
    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.0
        } else if index == 1 {
            &self.1
        } else {
            panic!("index out of bounds");
        }
    }
}

impl Mul<usize> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, rhs: usize) -> Self::Output {
        Vec2(self.0 * rhs as isize, self.1 * rhs as isize)
    }
}

impl Mul<isize> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: isize) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Vec2> for isize {
    type Output = Vec2;
    
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl Mul<Vec2> for usize {
    type Output = Vec2;
    
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2(self as isize * rhs.0, self as isize * rhs.1)
    }
}

impl Div<usize> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: usize) -> Self::Output {
        Vec2(self.0 / rhs as isize, self.1 / rhs as isize)
    }
}

impl Div<isize> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: isize) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl Vec2 {
    pub fn as_tuple(&self) -> (isize, isize) {
        (self.0, self.1)
    }
    
    pub fn as_tuple_unsigned(&self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
}