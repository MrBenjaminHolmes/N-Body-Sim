mod particle;
mod quadtree;
mod renderer;
mod simulation;
mod vector;
use macroquad::prelude::*;
use renderer::window_conf;
use simulation::simulate_step;

#[macroquad::main(window_conf())]
async fn main() {
    let dt = 2000.0;

    let mut particles = particle::random_particles(15);

    let mut show_tree_debug = false;
    let mut show_tree_outline = false;
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::D) {
            show_tree_debug = !show_tree_debug;
        }
        if is_key_pressed(KeyCode::V) {
            show_tree_outline = !show_tree_outline;
        }
        if is_key_pressed(KeyCode::Up) {
            unsafe {
                renderer::SCALE *= 1.1;
            }
        }

        if is_key_pressed(KeyCode::Down) {
            unsafe {
                renderer::SCALE /= 1.1;
            }
        }

        if is_key_down(KeyCode::Left) {
            unsafe {
                renderer::OFFSET_X += 5.0;
            }
        }

        if is_key_down(KeyCode::Right) {
            unsafe {
                renderer::OFFSET_X -= 5.0;
            }
        }
        let mut root = quadtree::init();

        for i in 0..particles.len() {
            root.insert_particle(i, &particles);
        }

        simulate_step(&mut particles, dt);

        renderer::render(&particles);
        if show_tree_debug {
            renderer::draw_tree_debug(&root);
        }
        if show_tree_outline {
            renderer::draw_quadtree(&root);
        }
        next_frame().await;
    }
}
