mod particle;
mod renderer;
mod simulation;
mod vector;

use macroquad::prelude::*;
use particle::Particle;
use simulation::simulate_step;
use vector::Vector2;

#[macroquad::main("N-Body Simulation")]
async fn main() {
    let dt = 36000.0;

    let mut particles = vec![
        // Star
        Particle {
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass: 1.989e30,
        },
        // Planet 1
        Particle {
            position: Vector2 {
                x: 1.496e11,
                y: 0.0,
            },
            velocity: Vector2 { x: 0.0, y: 29785.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass: 5.972e24,
        },
        // Planet 2
        Particle {
            position: Vector2 {
                x: 2.279e11,
                y: 0.0,
            },
            velocity: Vector2 { x: 0.0, y: 24130.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass: 6.39e23,
        },
    ];

    let mut step = 0;

    loop {
        clear_background(BLACK);

        simulate_step(&mut particles, dt);

        renderer::render(&particles);

        // 3. Optional debug logging
        if step % 10 == 0 {
            println!("Step {}", step);
            for (i, p) in particles.iter().enumerate() {
                println!("Particle {}: ({}, {})", i, p.position.x, p.position.y);
            }
        }
        step += 1;

        next_frame().await;
    }
}
