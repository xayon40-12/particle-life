use crate::particle::Particle;
use crate::utils::dir;
use ggez::event::{self, KeyCode};
use ggez::graphics::{self, Color, MeshBuilder};
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use rand::prelude::*;

pub struct World<const N: usize> {
    size: [f32; 2],
    p: Vec2,
    speed: f32,
    theta: f32,
    aspeed: f32,
    interactions: [[f32; N]; N],
    damping: f32,
    colors: [Color; N],
    particles: Vec<Particle>,
}

impl<'a, const N: usize> World<N> {
    pub fn new(size: [f32; 2], interactions: [[f32; N]; N], damping: f32) -> World<N> {
        let particles = vec![];
        World {
            size,
            p: Vec2::ZERO,
            speed: 0.01,
            theta: 0.0,
            aspeed: 0.02,
            interactions,
            damping,
            colors: [Color::WHITE; N],
            particles,
        }
    }

    pub fn initialize_particles(&mut self, num: usize, d: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..N {
            self.colors[i] = (rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()).into();
        }
        for _ in 0..num {
            let x = rng.gen::<f32>() * self.size[0];
            let y = rng.gen::<f32>() * self.size[1];
            let p = Vec2::new(x, y);
            let id = rng.gen::<usize>() % N;
            self.particles.push(Particle::new(id, p, d))
        }
    }

    fn handle_events(&mut self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::Up)
            || keyboard::is_key_pressed(ctx, KeyCode::Comma)
        {
            self.p += self.speed * dir(self.theta);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) || keyboard::is_key_pressed(ctx, KeyCode::O)
        {
            self.p -= self.speed * dir(self.theta);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) || keyboard::is_key_pressed(ctx, KeyCode::A)
        {
            self.theta += self.aspeed;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right)
            || keyboard::is_key_pressed(ctx, KeyCode::E)
        {
            self.theta -= self.aspeed;
        }
    }
}

impl<const N: usize> event::EventHandler<ggez::GameError> for World<N> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let w = self.size[0];
        let w2 = w * 0.5;
        let h = self.size[1];
        let h2 = h * 0.5;
        let dt = 1.0 / 60.0;
        self.handle_events(ctx);
        let dist = |a: &Vec2, b: &Vec2| {
            let mut dx = b.x - a.x;
            if dx.abs() > w2 {
                dx = -dx.signum() * (w - dx.abs());
            }
            let mut dy = b.y - a.y;
            if dy.abs() > h2 {
                dy = -dy.signum() * (h - dy.abs());
            }
            let d = Vec2::new(dx, dy);
            let n = d.length();
            (d / (n + 1e-14), n)
        };
        let s = self.particles.len();
        for j in 0..s {
            for i in 0..s {
                if j != i {
                    let p2 = self.particles[i].clone();
                    let p1 = &mut self.particles[j];
                    p1.interact(
                        &p2,
                        self.interactions[p1.id()][p2.id()],
                        &dist,
                        self.damping,
                    );
                }
            }
        }
        for p in &mut self.particles {
            p.update(dt, &self.size);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.1, 1.0].into());

        let mut mb = MeshBuilder::new();
        for p in &self.particles {
            mb.circle(
                graphics::DrawMode::fill(),
                p.pos(),
                5.0,
                1.0,
                self.colors[p.id()],
            )?;
        }

        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, ([self.p.x, self.p.y],))?;
        graphics::present(ctx)?;
        Ok(())
    }
}
