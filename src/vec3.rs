use rand::{thread_rng, Rng};

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

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();

        Vec3 {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max)
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.squared_norm() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
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
        self.squared_norm().sqrt()
    }

    pub fn squared_norm(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
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

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return 
            self.x().abs() < s &&
            self.y().abs() < s &&
            self.z().abs() < s
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

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x() * rhs.x(),
            y: self.y() * rhs.y(),
            z: self.z() * rhs.z()
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

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
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

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x(),
            y: -self.y(),
            z: -self.z()
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
