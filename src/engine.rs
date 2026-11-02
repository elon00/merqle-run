//! Local-only deterministic bit-mixing example.
//! This is NOT Conway's Game of Life, cryptography, or a network consensus engine.
use std::sync::atomic::{AtomicU64, Ordering};

pub struct BitMixerEngine {
    state: AtomicU64,
}

fn transform(value: u64) -> u64 {
    (value.rotate_left(1) ^ value.rotate_right(1)) | 0x0101010101010101
}

impl BitMixerEngine {
    pub fn new(seed: u64) -> Self {
        Self {
            state: AtomicU64::new(seed),
        }
    }

    pub fn step(&self) -> u64 {
        let mut observed = self.state.load(Ordering::Acquire);
        loop {
            let next = transform(observed);
            match self.state.compare_exchange_weak(
                observed,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return next,
                Err(current) => observed = current,
            }
        }
    }

    pub fn state(&self) -> u64 {
        self.state.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::{transform, BitMixerEngine};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn one_step_matches_reference_transform() {
        let engine = BitMixerEngine::new(0x55AA55AA55AA55AA);
        assert_eq!(engine.step(), transform(0x55AA55AA55AA55AA));
        assert_eq!(engine.state(), transform(0x55AA55AA55AA55AA));
    }

    #[test]
    fn concurrent_updates_are_not_lost() {
        let engine = Arc::new(BitMixerEngine::new(0x123456789ABCDEF0));
        let mut workers = Vec::new();
        for _ in 0..4 {
            let shared = Arc::clone(&engine);
            workers.push(thread::spawn(move || {
                for _ in 0..8 {
                    shared.step();
                }
            }));
        }
        for worker in workers {
            worker.join().expect("worker panicked");
        }
        let mut expected = 0x123456789ABCDEF0;
        for _ in 0..32 {
            expected = transform(expected);
        }
        assert_eq!(engine.state(), expected);
    }
}
