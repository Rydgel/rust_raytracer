use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Default)]
pub struct Vec3([f32; 3]);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f32 {
        self[0]
    }
    pub fn y(&self) -> f32 {
        self[1]
    }
    pub fn z(&self) -> f32 {
        self[2]
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn len(&self) -> f32 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, othr: Vec3) -> Self::Output {
        Self([
            self.x() + othr.x(),
            self.y() + othr.y(),
            self.z() + othr.z(),
        ])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, othr: Vec3) {
        self[0] += othr[0];
        self[1] += othr[1];
        self[2] += othr[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, othr: Vec3) -> Self::Output {
        Self([
            self.x() - othr.x(),
            self.y() - othr.y(),
            self.z() - othr.z(),
        ])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, othr: Vec3) -> Self::Output {
        Self([
            self.x() * othr.x(),
            self.y() * othr.y(),
            self.z() * othr.z(),
        ])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, othr: f32) -> Self::Output {
        Self([self.x() * othr, self.y() * othr, self.z() * othr])
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, othr: Vec3) -> Self::Output {
        othr * self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, othr: Vec3) {
        self[0] *= othr[0];
        self[1] *= othr[1];
        self[2] *= othr[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, othr: f32) {
        self[0] *= othr;
        self[1] *= othr;
        self[2] *= othr;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, othr: Vec3) -> Self::Output {
        Self([
            self.x() / othr.x(),
            self.y() / othr.y(),
            self.z() / othr.z(),
        ])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, othr: f32) -> Self::Output {
        Self([self.x() / othr, self.y() / othr, self.z() / othr])
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, othr: Vec3) -> Self::Output {
        othr / self
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, othr: Vec3) {
        self[0] /= othr[0];
        self[1] /= othr[1];
        self[2] /= othr[2];
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, othr: f32) {
        self[0] /= othr;
        self[1] /= othr;
        self[2] /= othr;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self([-self.x(), -self.y(), -self.z()])
    }
}
