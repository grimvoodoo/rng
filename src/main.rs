use std::time::{SystemTime, UNIX_EPOCH};

struct SimpleLCG {
    state: u32,
}

impl SimpleLCG {
    fn new(seed: u32) -> SimpleLCG {
        SimpleLCG { state: seed }
    }

    fn next(&mut self) -> u32 {
        let a: u32 = 1664525;
        let c: u32 = 1013904223;
        let m: u32 = 2u32.pow(16);
        self.state = (a.wrapping_mul(self.state).wrapping_add(c)) % m;
        self.state
    }
}
fn main() {
    let rng = random_number(10);
    println!("{:?}", rng)
}

fn random_number(max_number: u32) -> u32 {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let mut lcg = SimpleLCG::new(seed.try_into().unwrap());
    lcg.next() % max_number
}
