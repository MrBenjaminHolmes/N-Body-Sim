mod vector;
use vector::Vector2;
const G: f64 = 6.67430e-11;

struct Particle {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f64,
}

fn updateParticle(p: &mut Particle, dt: f64) {
    p.velocity = p.velocity + p.acceleration * dt;
    p.position = p.position + p.velocity * dt;
}
fn accel(force: Vector2, mass: f64) -> Vector2 {
    force / mass
}

fn grav_force(p1: &Particle, p2: &Particle) -> Option<Vector2> {
    let displacement = p2.position - p1.position;
    let distance = displacement.magnitude();

    if distance == 0.0 {
        return None;
    }

    let force_magnitude = G * p1.mass * p2.mass
        / (distance * distance);

    let direction = displacement.normalise();

    Some(direction * force_magnitude)
}


fn main() {
    let mut p = Particle {
    position: Vector2 { x: 0.0, y: 0.0 },
    velocity: Vector2 { x: 2.0, y: 3.0 },
    acceleration: Vector2 { x: 1.0, y: 0.0 },
    mass: 10.0,
};
    let mut p1 = Particle {
        position: Vector2 { x: 10.0, y: 0.0 },
        velocity: Vector2 { x: 2.0, y: 3.0 },
        acceleration: Vector2 { x: 1.0, y: 0.0 },
        mass: 10.0,
    };

    let grav_force_vec = grav_force(&p, &p1);

    match grav_force_vec {
        Some(force) => {
            println!("Gravitational force: ({}, {})", force.x, force.y);
            p.acceleration = accel(force, p.mass);

            let opposite_force = force * -1.0;
            p1.acceleration = accel(opposite_force, p1.mass);
            
            println!("Particle 1 acceleration: ({}, {})",
            p.acceleration.x, p.acceleration.y);

            println!("Particle 2 acceleration: ({}, {})",
            p1.acceleration.x, p1.acceleration.y);
        }
        None => {
            println!("Cannot calculate gravitational force: particles overlap");
        }
    }

    

}



