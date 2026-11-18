use crate::{particle::Particle, quadtree::Node, vector::Vector2};
use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "N-Body Simulation".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

pub fn render(particles: &[Particle]) {
    let scale: f32 = 2e-9;

    for particle in particles {
        let screen_x = screen_width() / 2.0 + particle.position.x as f32 * scale;

        let screen_y = screen_height() / 2.0 - particle.position.y as f32 * scale;

        draw_circle(screen_x, screen_y, 5.0, WHITE);
    }
}
pub fn draw_square(top_left: Vector2, size: f64) {
    let scale: f32 = 2e-9;

    let screen_size = size as f32 * scale;

    let screen_x = screen_width() / 2.0 + top_left.x as f32 * scale;

    let screen_y = screen_height() / 2.0 - top_left.y as f32 * scale;

    draw_rectangle_lines(screen_x, screen_y, screen_size, screen_size, 2.0, WHITE);
}
pub fn draw_quadtree(node: &Node) {
    draw_square(node.top_left, node.size as f64);

    if let Some(children) = &node.children {
        for child in children {
            draw_quadtree(child);
        }
    }
}
pub fn draw_tree_debug(root: &Node) {
    // Background panel
    draw_rectangle(
        10.0,
        10.0,
        480.0,
        screen_height() - 20.0,
        Color::from_rgba(0, 0, 0, 220),
    );

    let mut y = 30.0;
    draw_node_debug(root, 0, "ROOT", &mut y);
}

fn draw_node_debug(node: &Node, depth: usize, name: &str, y: &mut f32) {
    if *y > screen_height() - 20.0 {
        return;
    }

    let indent = "  ".repeat(depth);

    let child_count = match &node.children {
        Some(_) => 4,
        None => 0,
    };

    let text = format!(
        "{}{} | depth: {} | size: {:.2e} | TL: ({:.2e}, {:.2e}) | particles: {:?} | children: {}",
        indent,
        name,
        depth,
        node.size,
        node.top_left.x,
        node.top_left.y,
        node.particles,
        child_count
    );

    draw_text(&text, 20.0, *y, 16.0, GREEN);

    *y += 20.0;

    if let Some(children) = &node.children {
        let names = ["TL", "TR", "BL", "BR"];

        for (i, child) in children.iter().enumerate() {
            draw_node_debug(child, depth + 1, names[i], y);
        }
    }
}
