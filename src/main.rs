use ring::digest::*;
use std::time::Instant;
use time;
use xorshift::{Rand, Rng, SeedableRng, SplitMix64, Xoroshiro128};

fn simple_sha2_256(value: &[u8]) {
    let mut sm: SplitMix64 = SeedableRng::from_seed(time::precise_time_ns());
    let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

    let buffer: &mut [u8] = &mut [0; 64];
    buffer[32..].copy_from_slice(value);

    let mut last_difficulty: u32 = 0;

    let mut last_write_time = Instant::now();
    let mut loop_count: u64 = 0;

    loop {
        loop_count += 1;

        buffer[0..8].copy_from_slice(&rng.next_u64().to_ne_bytes());
        buffer[8..16].copy_from_slice(&rng.next_u64().to_ne_bytes());
        buffer[16..24].copy_from_slice(&rng.next_u64().to_ne_bytes());
        buffer[24..32].copy_from_slice(&rng.next_u64().to_ne_bytes());

        let result = digest(&SHA256, &buffer);
        let mut current_difficulty: u32 = 0;

        let result_u32 = result.as_ref();
        for element in result_u32 {
            let zeros = element.leading_zeros();
            current_difficulty += zeros;

            if zeros < 8 {
                break;
            }
        }

        if last_difficulty < current_difficulty {
            println!("{} {}", current_difficulty, base64::encode(&buffer[0..32]));
            last_difficulty = current_difficulty;
        }

        if loop_count % 10000000 == 0 {
            let timespan = last_write_time.elapsed();
            println!("{}s", timespan.as_secs_f64());
            last_write_time = Instant::now();
        }
    }
}

fn main() {
    let buffer: &mut [u8] = &mut [0; 32];
    simple_sha2_256(buffer);
}
