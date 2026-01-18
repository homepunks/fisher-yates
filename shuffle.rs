use std::time::{UNIX_EPOCH, SystemTime};

fn main() {
    const SZ: usize = 37;
    let mut array: [usize; SZ] = core::array::from_fn(|i| i);
    println!("Sorted:   {:?}", array);

    shuffle(&mut array);
    println!("Shuffled: {:?}", array);
}

fn shuffle<T: Ord + Copy>(collection: &mut [T]) {
    for i in (1..collection.len()).rev() {
	let j = prng(i as u32);
	collection.swap(i as usize, j as usize);
    }
}

/* feel free to replace with whatever
   CSPRNG of your liking              */
fn prng(limit: u32) -> i32 {
    let nanos = SystemTime::now()
	.duration_since(UNIX_EPOCH)
	.unwrap()
	.subsec_nanos();

    (nanos % limit) as i32
}
