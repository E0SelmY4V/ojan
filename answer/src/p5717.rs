use ojan::*;
use std::iter::Iterator;

struct Triangle(u32, u32, u32);

impl Triangle {
    pub fn create(lines: impl Iterator<Item = u32>) -> Option<Triangle> {
        let mut lines: Vec<_> = lines.take(3).collect();
        lines.sort();
        if lines[0] + lines[1] > lines[2] {
            Some(Triangle(lines[0], lines[1], lines[2]))
        } else {
            None
        }
    }
    pub fn print_shape(&self) {
        let &Triangle(a, b, c) = self;
        let cc = c * c;
        let info = match a * a + b * b {
            v if v > cc => "Acute triangle",
            v if v < cc => "Obtuse triangle",
            _ => "Right triangle",
        };
        println!("{}", info);
    }
    pub fn print_equ(&self) {
        let &Triangle(a, b, c) = self;
        let f1 = a == b;
        let f2 = b == c;
        if f1 || f2 {
            println!("Isosceles triangle");
            if f1 && f2 {
                println!("Equilateral triangle");
            }
        }
    }
}

pub fn main() {
    let mut iner = input::new();
    if let Some(triangle) = Triangle::create(iner.line().parse_to_iter()) {
        triangle.print_shape();
        triangle.print_equ();
    } else {
        println!("Not triangle");
    }
}
