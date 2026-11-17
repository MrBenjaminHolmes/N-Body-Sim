use crate::particle::{accel, grav_force, update_particle, Particle};
use crate::vector::Vector2;

pub fn simulate_step(particles: &mut Vec<Particle>, dt: f64) {
    // Reset accelerations to zero
    let mut accelerations = vec![Vector2 { x: 0.0, y: 0.0 }; particles.len()];

    // Calculate gravitational accelerations.
    for i in 0..particles.len() {
        for j in 0..particles.len() {
            if i == j {
                continue;
            }

            if let Some(force) = grav_force(&particles[i], &particles[j]) {
                accelerations[i] = accelerations[i] + accel(force, particles[i].mass);
            }
        }
    }
    // Apply accelerations.
    for (particle, acceleration) in particles.iter_mut().zip(accelerations) {
        particle.acceleration = acceleration;
    }

    // Update particle positions and velocities.
    for particle in particles.iter_mut() {
        update_particle(particle, dt);
    }
}
