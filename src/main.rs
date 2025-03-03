use std::{ops::ControlFlow, sync::{Arc, Mutex}, time::SystemTime};
use rayon::prelude::*;
use sha256::digest;
use std::env;

fn main() {
	if env::args().len() != 5 {
        println!("Wrong arguments");
        return;
    }
    let args = env::args().collect::<Vec<String>>();
    let mut zeros_count: Option<usize> = None;
    let mut to_find_count: Option<usize> = None;
    [1usize, 3usize].iter().for_each(|idx| {
        let arg = &args[*idx];
        let arg2 = args[*idx + 1].parse::<usize>().expect(&format!("Argument {} must be a number", idx + 1));
        if arg == "-F" { to_find_count = Some(arg2) }
        else if arg == "-N" { zeros_count = Some(arg2) }
        else { panic!("Wrong argument: {}", arg); };
    });
    let zeros_count = zeros_count.expect("Wrong zeros count");
    let to_find_count = to_find_count.expect("Wrong to find count");
    let found_count = Arc::new(Mutex::new(0));
    let zeros = "0".repeat(zeros_count);

    println!("---- BEGIN ----");
    let time_start = SystemTime::now();

    (0..10).into_par_iter().try_for_each(|thrd| {
        let found_count_clone = Arc::clone(&found_count);
        (1_000_000 * thrd .. 1_000_000 * (thrd+1)).into_iter().try_for_each(|num| {
            let dg = digest(num.to_string());
            if dg.ends_with(&zeros) {
                println!("{}\t{}", num, dg);
                let mut fc_locked = found_count_clone.lock().unwrap();
                //println!("Thread {}, fc_locked = {}", thrd, *fc_locked);
                *fc_locked += 1;
                if *fc_locked == to_find_count { return ControlFlow::Break(()) };
            };
            ControlFlow::Continue(())
            });
            let mut fc_locked = found_count_clone.lock().unwrap();
            if *fc_locked == to_find_count { ControlFlow::Break(()) } else { ControlFlow::Continue(()) };
        });
    println!("Time elapsed = {:?}", time_start.elapsed().unwrap());
    println!("---- END ----");
}
