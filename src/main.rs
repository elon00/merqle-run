mod engine;

use engine::BitMixerEngine;

fn main() {
    let engine = BitMixerEngine::new(0x55AA55AA55AA55AA);
    for _ in 0..10 {
        engine.step();
    }
    println!(
        "Local deterministic bit-mixing demonstration: {:016X}",
        engine.state()
    );
    println!("No RPC, PQC, Conway Game of Life, settlement, or deployment verification performed.");
}
