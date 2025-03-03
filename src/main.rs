use std::time::SystemTime;
use rayon::prelude::*;

use sha256::digest;

fn main() {
    let zeros_count = 5;
    let zeros = "0".repeat(zeros_count);
    println!("---- BEGIN ----");
    let time_start = SystemTime::now();

    (0..10).into_par_iter().for_each(|thrd| {
        (1_000_000 * thrd .. 1_000_000 * (thrd+1)).into_iter().for_each(|num| {
            let dg = digest(num.to_string());
            if dg.ends_with(&zeros) { println!("{}\t{}", num, dg) };
        });
    });
    println!("Time elapsed = {:?}", time_start.elapsed().unwrap());
    println!("---- END ----");
}
