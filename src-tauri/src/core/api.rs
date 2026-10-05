use std::collections::HashSet;

use rand::Rng;

pub const MAX_ID: i32 = 10_000_000;

pub fn generate_id(used: &HashSet<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    loop {
        let id = rng.gen_range(1..=MAX_ID);
        if !used.contains(&id) {
            return id;
        }
    }
}
