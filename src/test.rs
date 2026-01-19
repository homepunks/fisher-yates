mod shuffle;

use shuffle::{shuffle_fisher_yates, shuffle_rand_crate};
use std::time::{Duration, Instant};

fn main() {
    const SZ: usize = 67;
    let base: [usize; SZ] = core::array::from_fn(|i| i);
    println!("Sorted:   {:?}\n\n", base);

    println!("<testing Fisher-Yates>");
    let mut array1 = base.clone();
    let dur1 = record_time(|| shuffle_fisher_yates(&mut array1));
    println!("Shuffled: {:?}", array1);
    println!("Duration: {}", duration_str(dur1));

    println!();

    println!("<testing rand crate>");
    let mut array2 = base.clone();
    let dur2 = record_time(|| shuffle_rand_crate(&mut array2));
    println!("Shuffled: {:?}", array2);
    println!("Duration: {}", duration_str(dur2));

    println!("\n{:?} vs {:?}", dur1, dur2);
    println!("think now...");
}

fn record_time<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}

fn duration_str(d: Duration) -> String {
    format!("{} ns", d.as_nanos())
}
