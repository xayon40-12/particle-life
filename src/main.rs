use ggez::event;
use ggez::input::mouse;
use ggez::GameResult;
use particle_life::world::World;

pub fn main() -> GameResult {
    let w = 1000.0;
    let h = 1000.0;
    let size = [w, h];
    let cb = ggez::ContextBuilder::new("Particle Life", "xayon40-12")
        .window_setup(ggez::conf::WindowSetup {
            title: "Particle Life".to_owned(),
            ..Default::default()
        })
        .window_mode(ggez::conf::WindowMode {
            width: w,
            height: h,
            ..Default::default()
        });
    let (mut ctx, event_loop) = cb.build()?;

    // ----------------------- parameters -----------------------

    let damping = 1.0 - 1e-4; // velocity damping
    let d = 300.0; // total interaction distance
    let nb = 1000; // number of particles

    // Interaction matrix. Must be a square matrix.
    // Its size NxN represent the numbre N of families (colors).
    // Positive coefficient correspond to attraction and negative to repultion.
    let interactions = [
        [2.0, 1.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 2.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 2.0, 1.0],
        [0.0, 0.0, 0.0, 0.0, 2.0],
    ];
    // let interactions = [
    //     [2.0, 1.0, 0.0, 0.0, 0.0],
    //     [-1.0, 2.0, 0.0, 0.0, 0.0],
    //     [0.0, 0.0, 0.0, 0.0, 0.0],
    //     [0.0, 0.0, 0.0, 0.0, 0.0],
    //     [0.0, 0.0, 0.0, 0.0, 0.0],
    // ];
    // let interactions = [
    //     [2.0, 1.0, -1.0, 1.0, -1.0],
    //     [-1.0, 2.0, 1.0, -1.0, 1.0],
    //     [1.0, -1.0, 2.0, 1.0, -1.0],
    //     [-1.0, 1.0, -1.0, 2.0, 1.0],
    //     [1.0, -1.0, 1.0, -1.0, 2.0],
    // ];
    // let interactions = [[-1.0, 1.0, 0.0], [1.0, -1.0, 10.0], [0.0, 10.0, 10.0]];

    // ----------------------------------------------------------

    let mut state = World::new(size, interactions, damping);
    state.initialize_particles(nb, d);
    mouse::set_cursor_grabbed(&mut ctx, true)?;
    mouse::set_cursor_hidden(&mut ctx, true);
    event::run(ctx, event_loop, state)
}
