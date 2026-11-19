use crate::particle::Particle;
use crate::vector::Vector2;
pub struct Node {
    pub top_left: Vector2,
    pub size: f64,
    pub depth: usize,
    pub children: Option<[Box<Node>; 4]>,
    pub particles: Vec<usize>,
    pub total_mass: f64,
    pub centre_of_mass: Vector2,
}

pub fn init() -> Node {
    Node {
        top_left: Vector2 {
            x: -2.0e11,
            y: 2.0e11,
        },
        size: 4.0e11,
        depth: 0,
        children: None,
        particles: Vec::new(),
        total_mass: 0.0,
        centre_of_mass: Vector2 { x: 0.0, y: 0.0 },
    }
}

impl Node {
    pub fn contains_particle(&self, particle: &Particle) -> bool {
        let right = self.top_left.x + self.size;
        let bottom = self.top_left.y - self.size;

        particle.position.x >= self.top_left.x
            && particle.position.x < right
            && particle.position.y <= self.top_left.y
            && particle.position.y > bottom
    }
    pub fn particle_index(&self, particle: &Particle) -> usize {
        let mid_x = self.top_left.x + self.size / 2.0;
        let mid_y = self.top_left.y - self.size / 2.0;

        if particle.position.x < mid_x {
            if particle.position.y >= mid_y {
                0 // top-left
            } else {
                2 // bottom-left
            }
        } else {
            if particle.position.y >= mid_y {
                1 // top-right
            } else {
                3 // bottom-right
            }
        }
    }
    pub fn subdivide(&mut self) {
        let half = self.size / 2.0;
        let child_depth = self.depth + 1;

        let top_left = Node {
            top_left: self.top_left,
            size: half,
            depth: child_depth,
            children: None,
            particles: Vec::new(),
            total_mass: 0.0,
            centre_of_mass: Vector2 { x: 0.0, y: 0.0 },
        };

        let top_right = Node {
            top_left: Vector2 {
                x: self.top_left.x + half,
                y: self.top_left.y,
            },
            size: half,
            depth: child_depth,
            children: None,
            particles: Vec::new(),
            total_mass: 0.0,
            centre_of_mass: Vector2 { x: 0.0, y: 0.0 },
        };

        let bottom_left = Node {
            top_left: Vector2 {
                x: self.top_left.x,
                y: self.top_left.y - half,
            },
            size: half,
            depth: child_depth,
            children: None,
            particles: Vec::new(),
            total_mass: 0.0,
            centre_of_mass: Vector2 { x: 0.0, y: 0.0 },
        };

        let bottom_right = Node {
            top_left: Vector2 {
                x: self.top_left.x + half,
                y: self.top_left.y - half,
            },
            size: half,
            depth: child_depth,
            children: None,
            particles: Vec::new(),
            total_mass: 0.0,
            centre_of_mass: Vector2 { x: 0.0, y: 0.0 },
        };

        self.children = Some([
            Box::new(top_left),
            Box::new(top_right),
            Box::new(bottom_left),
            Box::new(bottom_right),
        ]);
    }
    pub fn insert_particle(&mut self, particle_index: usize, particles: &[Particle]) {
        let particle = &particles[particle_index];
        let contains_bool = self.contains_particle(particle);

        if contains_bool {
            let quadrant = self.particle_index(particle);
            if let Some(children) = &mut self.children {
                children[quadrant].insert_particle(particle_index, particles);
            } else {
                if self.particles.is_empty() {
                    // Empty leaf: just store the particle
                    self.particles.push(particle_index);
                } else {
                    // Occupied leaf: subdivide

                    let old_particle_index = self.particles[0];
                    self.particles.clear();

                    self.subdivide();

                    self.insert_particle(old_particle_index, particles);
                    self.insert_particle(particle_index, particles);
                }
            }
        }
    }
    pub fn calculate_mass_distribution(&mut self, particles: &[Particle]) {
        self.total_mass = 0.0;
        self.centre_of_mass = Vector2 { x: 0.0, y: 0.0 };
        if let Some(children) = &mut self.children {
            let mut weighted_x = 0.0;
            let mut weighted_y = 0.0;
            for child in children.iter_mut() {
                child.calculate_mass_distribution(particles);
                self.total_mass += child.total_mass;
                weighted_x += child.total_mass * child.centre_of_mass.x;
                weighted_y += child.total_mass * child.centre_of_mass.y;
            }
            if self.total_mass > 0.0 {
                let com_x = weighted_x / self.total_mass;
                let com_y = weighted_y / self.total_mass;
                self.centre_of_mass = Vector2 { x: com_x, y: com_y };
            }
        } else {
            //Leaf Node
            if !self.particles.is_empty() {
                let index = self.particles[0];
                let particle = &particles[index];
                self.total_mass = particle.mass;
                self.centre_of_mass = particle.position
            }
        }
    }
}
