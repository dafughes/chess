use engine::random::RandomEngine;

pub mod engine;
pub mod uci;

fn main() {
    engine::main_loop::<RandomEngine>();
}
