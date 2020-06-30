use std::ops;
use std::ops::Neg;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Self {
        Vec2{x: 0., y: 0.}
    }
    pub fn size(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn unit(&self) -> Self {
        let size = self.size();
        // if size == 0. {
            // Vec2{x: 1., y: 0.}
        // } else {
            Vec2{x: self.x / size, y: self.y/size}
        // }
    }
    pub fn update(&mut self, from: &Vec2) {
        self.x = from.x;
        self.y = from.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2{x: -self.x, y: -self.y}
    }
}

impl_op_ex!(+ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x + b.x, y: a.y + b.y} });
impl_op_ex!(- |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x - b.x, y: a.y - b.y} });
impl_op_ex!(* |a: &Vec2, b: f32| -> Vec2 { Vec2{x: a.x * b, y: a.y * b } });
impl_op_ex!(* |b: f32, a: &Vec2| -> Vec2 { Vec2{x: a.x * b, y: a.y * b } });