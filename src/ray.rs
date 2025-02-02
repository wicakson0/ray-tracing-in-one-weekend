use crate::vec3::Vec3;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn orig(&self) -> Point3 {
        self.orig
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + t * self.dir;
    }
}