use std::ops;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y:f64, z:f64) -> Self {
        Vec3 { e:[x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        return self.e[0] * v.e[0]
             + self.e[1] * v.e[1]
             + self.e[2] * v.e[2];
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.e[0] + rhs.e[0];
        let y = self.e[1] + rhs.e[1];
        let z = self.e[2] + rhs.e[2];
        Vec3::new(x, y, z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        let x = self.e[0] + rhs.e[0];
        let y = self.e[1] + rhs.e[1];
        let z = self.e[2] + rhs.e[2];
        *self = Vec3::new(x, y, z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.e[0] - rhs.e[0];
        let y = self.e[1] - rhs.e[1];
        let z = self.e[2] - rhs.e[2];
        Vec3::new(x, y, z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        let x = self.e[0] - rhs.e[0];
        let y = self.e[1] - rhs.e[1];
        let z = self.e[2] - rhs.e[2];
        *self = Vec3::new(x, y, z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.e[0] * rhs;
        let y = self.e[1] * rhs;
        let z = self.e[2] * rhs;
        Vec3::new(x, y, z)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.e[0] * rhs;
        let y = self.e[1] * rhs;
        let z = self.e[2] * rhs;
        Vec3::new(x, y, z)
    }
}


impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        let x = self.e[0] * rhs;
        let y = self.e[1] * rhs;
        let z = self.e[2] * rhs;
        *self = Vec3::new(x, y, z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let x = self * rhs.e[0];
        let y = self * rhs.e[1];
        let z = self * rhs.e[2];
        Vec3::new(x, y, z)
    }
}

impl ops::Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.e[0] * rhs.e[0];
        let y = self.e[1] * rhs.e[1];
        let z = self.e[2] * rhs.e[2];
        Vec3::new(x, y, z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
            let a = Vec3::new(1.0, 2.0, 3.0);
            let b = Vec3::new(5.0, 8.0, 13.0);
            let mut c = a + b;
        assert_eq!(c.e[0], 6.0);
        assert_eq!(c.e[1], 10.0);
        assert_eq!(c.e[2], 16.0);
        c += b;
        assert_eq!(c.e[0], 11.0);
        assert_eq!(c.e[1], 18.0);
        assert_eq!(c.e[2], 29.0);
    }
}

pub type Point3 = Vec3;

// Vector utils
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    let x = u.e[1] * v.e[2] - u.e[2] * v.e[1];
    let y = u.e[2] * v.e[0] - u.e[0] * v.e[2];
    let z = u.e[0] * v.e[1] - u.e[1] * v.e[0];
    Vec3::new(x, y, z)
}

