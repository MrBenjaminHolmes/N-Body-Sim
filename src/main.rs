mod particle;
mod simulation;
mod vector;
use particle::{accel, grav_force, update_particle, Particle};
use simulation::simulate_step;
use vector::Vector2;

fn main() {
    let dt = 0.05;

    let mut particles: Vec<Particle> = vec![
        Particle {
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 2.0, y: 3.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass: 10.0,
        },
        Particle {
            position: Vector2 { x: 10.0, y: 0.0 },
            velocity: Vector2 { x: 2.0, y: 3.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass: 10.0,
        },
    ];
    for step in 0..100 {
        simulate_step(&mut particles, dt);

        if step % 10 == 0 {
            println!("Step {}", step);

            for (i, particle) in particles.iter().enumerate() {
                println!(
                    "Particle {}: ({}, {})",
                    i, particle.position.x, particle.position.y
                );
            }
        }
    }
}
