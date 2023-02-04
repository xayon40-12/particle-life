use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Particle {
    id: usize,
    p: Vec2,
    v: Vec2,
    inter_dist: f32,
}

pub type Distance<'a> = &'a dyn Fn(&Vec2, &Vec2) -> (Vec2, f32);

impl Particle {
    pub fn new(id: usize, p: Vec2, inter_dist: f32) -> Particle {
        Particle {
            id,
            p,
            v: Vec2::new(0.0, 0.0),
            inter_dist,
        }
    }

    pub fn pos(&self) -> [f32; 2] {
        [self.p.x, self.p.y]
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn update(&mut self, dt: f32, size: &[f32; 2]) {
        self.p += self.v * dt;
        self.p.x = (self.p.x + size[0]) % size[0];
        self.p.y = (self.p.y + size[1]) % size[1];
    }

    pub fn interact(&mut self, other: &Self, i: f32, dist: Distance, damping: f32) {
        let inter = self.inter_dist;
        let (d, n) = dist(&self.p, &other.p);
        let c = if n < inter {
            // -(inter / (n + 1e-14)).powi(2)
            -1.0 + n / inter
        } else if n < 2.0 * inter {
            (n - inter) / inter * i
        } else if n < 3.0 * inter {
            (3.0 * inter - n) / inter * i
        } else {
            0.0
        };
        self.v += d * c;
        self.v *= damping;
    }
}
