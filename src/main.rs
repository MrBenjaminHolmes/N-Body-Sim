mod particle;
mod quadtree;
mod renderer;
mod simulation;
mod vector;
use crate::quadtree::init;
use macroquad::prelude::*;
use particle::Particle;
use renderer::window_conf;
use simulation::simulate_step;
use vector::Vector2;

#[macroquad::main(window_conf())]
async fn main() {
    let dt = 3600.0;

    let mut particles = vec![
        // Star
        Particle {
            position: Vector2 {
                x: -1.0e11,
                y: -1.0e11,
            },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            mass: 1.0e30,
        },
    ];

    let mut step = 0;
    let mut show_tree_debug = true;
    let mut root = quadtree::init();
    root.subdivide();
    let c = root.particle_index(&particles[0]);
    println!("Particle Index: {}", c);

    for (index, particle) in particles.iter().enumerate() {
        if let Some(children) = &mut root.children {
            for child in children.iter_mut() {
                if child.contains_particle(&particles[index]) {
                    //let index = root.particle_index(&particles[0]);
                    child.particles.push(index);
                    child.subdivide();
                }
            }
        }
    }

    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::D) {
            show_tree_debug = !show_tree_debug;
        }
        simulate_step(&mut particles, dt);

        renderer::draw_quadtree(&root);
        renderer::render(&particles);
        if show_tree_debug {
            renderer::draw_tree_debug(&root);
        }
        next_frame().await;
    }
}
