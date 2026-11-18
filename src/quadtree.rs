use crate::particle::Particle;
use crate::vector::Vector2;
pub struct Node {
    pub top_left: Vector2,
    pub size: f64,
    pub children: Option<[Box<Node>; 4]>,
    pub particles: Vec<usize>,
}

pub fn init() -> Node {
    Node {
        top_left: Vector2 {
            x: -2.0e11,
            y: 2.0e11,
        },
        size: 4.0e11,
        children: None,
        particles: Vec::new(),
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
        if particle.position.x <= 0.0 {
            if particle.position.y >= 0.0 {
                return 0;
            } else {
                return 2;
            }
        } else {
            if particle.position.y > 0.0 {
                return 1;
            } else {
                return 3;
            }
        }
    }
    pub fn subdivide(&mut self) {
        let half = self.size / 2.0;

        let top_left = Node {
            top_left: self.top_left,
            size: half,
            children: None,
            particles: Vec::new(),
        };
        let top_right = Node {
            top_left: Vector2 {
                x: self.top_left.x + half,
                y: self.top_left.y,
            },
            size: half,
            children: None,
            particles: Vec::new(),
        };
        let bottom_left = Node {
            top_left: Vector2 {
                x: self.top_left.x,
                y: self.top_left.y - half,
            },
            size: half,
            children: None,
            particles: Vec::new(),
        };

        let bottom_right = Node {
            top_left: Vector2 {
                x: self.top_left.x + half,
                y: self.top_left.y - half,
            },
            size: half,
            children: None,
            particles: Vec::new(),
        };
        self.children = Some([
            Box::new(top_left),
            Box::new(top_right),
            Box::new(bottom_left),
            Box::new(bottom_right),
        ]);
    }
}
