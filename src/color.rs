use crate::vec3::Vec3;
use std::io::{self, Write};

pub(crate) type Color = Vec3;

impl Color {
    pub fn write_color<W: Write>(&self, out: &mut W) -> io::Result<()> {
        let r = self.x();
        let g = self.y();
        let b = self.z();
        
        let rbyte = (255.999 * r) as u8;
        let gbyte = (255.999 * g) as u8;
        let bbyte = (255.999 * b) as u8;

        writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
    }
}