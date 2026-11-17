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

pub fn subdivide(parent: &mut Node) {
    let half = parent.size / 2.0;

    let top_left = Node {
        top_left: parent.top_left,
        size: half,
        children: None,
        particles: Vec::new(),
    };
    let top_right = Node {
        top_left: Vector2 {
            x: parent.top_left.x + half,
            y: parent.top_left.y,
        },
        size: half,
        children: None,
        particles: Vec::new(),
    };
    let bottom_left = Node {
        top_left: Vector2 {
            x: parent.top_left.x,
            y: parent.top_left.y - half,
        },
        size: half,
        children: None,
        particles: Vec::new(),
    };

    let bottom_right = Node {
        top_left: Vector2 {
            x: parent.top_left.x + half,
            y: parent.top_left.y - half,
        },
        size: half,
        children: None,
        particles: Vec::new(),
    };
    parent.children = Some([
        Box::new(top_left),
        Box::new(top_right),
        Box::new(bottom_left),
        Box::new(bottom_right),
    ]);
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
}
