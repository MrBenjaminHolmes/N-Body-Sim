use crate::particle::{accel, grav_force, update_particle, Particle};
use crate::vector::Vector2;

pub fn simulate_step(particles: &mut Vec<Particle>, dt: f64) {
    // Reject invalid time steps.
    if !dt.is_finite() || dt <= 0.0 {
        return;
    }

    let mut accelerations = vec![Vector2 { x: 0.0, y: 0.0 }; particles.len()];

    // Calculate gravitational accelerations.
    for i in 0..particles.len() {
        // Avoid division by zero or invalid mass.
        let mass = particles[i].mass;

        if !mass.is_finite() || mass <= 0.0 {
            continue;
        }

        for j in 0..particles.len() {
            if i == j {
                continue;
            }

            // Skip invalid source particles.
            let other_mass = particles[j].mass;

            if !other_mass.is_finite() || other_mass <= 0.0 {
                continue;
            }

            if let Some(force) = grav_force(&particles[i], &particles[j]) {
                // Prevent invalid force values from propagating.
                if !force.x.is_finite() || !force.y.is_finite() {
                    continue;
                }

                let acceleration = accel(force, mass);

                if acceleration.x.is_finite() && acceleration.y.is_finite() {
                    accelerations[i] = accelerations[i] + acceleration;
                }
            }
        }
    }

    // Apply accelerations.
    for (particle, acceleration) in particles.iter_mut().zip(accelerations) {
        particle.acceleration = acceleration;
    }

    // Update particle positions and velocities.
    for particle in particles.iter_mut() {
        if particle.mass.is_finite() && particle.mass > 0.0 {
            update_particle(particle, dt);
        }
    }
}
