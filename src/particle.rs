use crate::vector::Vector2;
const G: f64 = 6.6743e-11;
pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub mass: f64,
}

pub fn update_particle(p: &mut Particle, dt: f64) {
    p.velocity = p.velocity + p.acceleration * dt;
    p.position = p.position + p.velocity * dt;
}

pub fn accel(force: Vector2, mass: f64) -> Vector2 {
    force / mass
}
pub fn grav_force(p1: &Particle, p2: &Particle) -> Option<Vector2> {
    let displacement = p2.position - p1.position;
    let distance = displacement.magnitude();

    if distance == 0.0 {
        return None;
    }

    let force_magnitude = G * p1.mass * p2.mass / (distance * distance);

    let direction = displacement.normalise();

    Some(direction * force_magnitude)
}
