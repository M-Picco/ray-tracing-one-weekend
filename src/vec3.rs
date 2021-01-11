#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let norm: f64 = self.norm();
        Vec3 { 
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        return 
            self.x() * other.x() +
            self.y() * other.y() +
            self.z() * other.z()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { 
            x: self.x() + other.x(), 
            y: self.y() + other.y(), 
            z: self.z() + other.z()
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x() - other.x(), 
            y: self.y() - other.y(), 
            z: self.z() - other.z()
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x() / rhs,
            y: self.y() / rhs,
            z: self.z() / rhs
        }
    }
}

impl std::ops::Div<usize> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: usize) -> Vec3 {
        self * (1.0 / rhs as f64)
    }
}
