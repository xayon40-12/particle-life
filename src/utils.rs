use glam::Vec2;

pub fn dir(a: f32) -> Vec2 {
    Vec2::new(a.cos(), a.sin())
}
