use std::time::SystemTime;
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
    let mut found_count = 0;
    let zeros = "0".repeat(zeros_count);

    println!("---- BEGIN ----");
    let time_start = SystemTime::now();

    (0..10).into_par_iter().for_each(|thrd| {
        (1_000_000 * thrd .. 1_000_000 * (thrd+1)).into_iter().try_for_each(|num| {
            let dg = digest(num.to_string());
            if dg.ends_with(&zeros) {
                println!("{}\t{}", num, dg);
                found_count += 1;
                if found_count == to_find_count { return ControlFlow::Break(()) };
            };
            ControlFlow::Continue(())
            });
    });
    println!("Time elapsed = {:?}", time_start.elapsed().unwrap());
    println!("---- END ----");
}
