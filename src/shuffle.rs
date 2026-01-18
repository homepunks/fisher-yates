use std::time::{UNIX_EPOCH, SystemTime};
use rand::prelude::SliceRandom;
use rand::rng;

pub fn shuffle_fisher_yates<T: Ord + Copy>(collection: &mut [T]) {
    for i in (1..collection.len()).rev() {
	let j = prng(i as u32);
	collection.swap(i as usize, j as usize);
    }
}

pub fn shuffle_rand_crate<T: Ord + Copy>(collection: &mut [T]) {
    let mut rng = rng();
    collection.shuffle(&mut rng);
}

// feel free to replace with whatever *RNG of your liking
fn prng(limit: u32) -> i32 {
    let nanos = SystemTime::now()
	.duration_since(UNIX_EPOCH)
	.unwrap()
	.subsec_nanos();

    (nanos % limit) as i32
}

