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
    let dt = 20.0;

    let mut particles = particle::random_particles(4);

    let mut show_tree_debug = true;
    let mut show_tree_outline = true;
    let mut show_tree_colour = false;

    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::D) {
            show_tree_debug = !show_tree_debug;
        }
        if is_key_pressed(KeyCode::V) {
            show_tree_outline = !show_tree_outline;
        }
        if is_key_pressed(KeyCode::C) {
            show_tree_colour = !show_tree_colour;
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
            root.calculate_mass_distribution(&particles);
        }

        simulate_step(&mut particles, dt);

        renderer::render(&particles);
        if show_tree_debug {
            renderer::draw_tree_debug(&root);
        }
        if show_tree_outline {
            renderer::draw_quadtree(&root);
        }
        if show_tree_colour {
            renderer::draw_quadtree_coloured(&root);
        }

        next_frame().await;
    }
}
