use crate::particle::Particle;
use macroquad::prelude::*;

pub fn render(particles: &[Particle]) {
    let scale: f32 = 1e-9;

    for particle in particles {
        let screen_x = screen_width() / 2.0 + particle.position.x as f32 * scale;

        let screen_y = screen_height() / 2.0 - particle.position.y as f32 * scale;

        draw_circle(screen_x, screen_y, 5.0, WHITE);
    }
}
