use std::ops::{Add, Sub, Mul,Div};

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}


impl Vector2 {
    pub fn magnitude(&self) -> f64 {
        (self.x* self.x + self.y* self.y).sqrt()
    }
    pub fn distance(self, other: Vector2) -> f64 {
        (((self.x-other.x)*(self.x-other.x))+((self.y-other.y)*(self.y-other.y))).sqrt()
    }
    pub fn normalise(&self) -> Vector2 {
    let mag = self.magnitude();

    if mag == 0.0 {
        return Vector2 { x: 0.0, y: 0.0 };
    }

    Vector2 {
        x: self.x / mag,
        y: self.y / mag,
    }
}
}


impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: f64) -> Vector2 {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl Div<f64> for Vector2 {
    type Output = Vector2;

    fn div(self, other: f64) -> Vector2 {
        Vector2 { x: (self.x/other), y: (self.y/other) }
    }
}