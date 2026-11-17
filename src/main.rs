mod particle;
mod quadtree;
mod renderer;
mod simulation;
mod vector;
use crate::quadtree::{init, subdivide};
use macroquad::prelude::*;
use particle::Particle;
use renderer::window_conf;
use simulation::simulate_step;
use vector::Vector2;

#[macroquad::main(window_conf())]
async fn main() {
    let dt = 36000.0;

    let mut particles = vec![
        // Star
        Particle {
            position: Vector2 {
                x: -1.0e11,
                y: 1.0e11,
            },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass: 1.0e20,
        },
    ];

    let mut step = 0;

    let mut root = quadtree::init();
    subdivide(&mut root);

    if let Some(children) = &mut root.children {
        for child in children.iter() {
            println!("{}", child.size);
        }
        subdivide(&mut children[0]);
        if let Some(children) = &mut children[0].children {
            for child in children.iter() {
                println!("{}", child.size);
            }
            subdivide(&mut children[0]);
        }
    }

    loop {
        clear_background(BLACK);

        simulate_step(&mut particles, dt);

        renderer::draw_quadtree(&root);
        renderer::render(&particles);

        next_frame().await;
    }
}
