use std::time::{SystemTime};
use std::io::prelude::*;
use std::fs::File;

// Utility function.
fn read_file(path : &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("No such file!"),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .ok()
        .expect("failed to read!");
    // Remove newline.
    contents.pop();
    return contents;
}

// Timing function.
fn time_func<F : FnOnce()>(times : &mut Vec<u64>, func : F) -> u64 {
    let now = SystemTime::now();
    func();
    let time = now.elapsed().unwrap().as_nanos();
    times.push(time as u64);
    return time as u64;
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let now = SystemTime::now();
    let mut times = Vec::with_capacity(50);

    let input = read_file("../input/day01");
    time_func(&mut times, || { day01::first(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);
    time_func(&mut times, || { day01::second(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);

    let input = read_file("../input/day02");
    time_func(&mut times, || { day02::first(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);
    time_func(&mut times, || { day02::second(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);

    let input = read_file("../input/day03");
    time_func(&mut times, || { day03::first(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);

    let low = 240298;
    let high = 784956;
    time_func(&mut times, || { day04::first(low, high); });
    println!("  Took {}", times.last().unwrap() / 1000000);
    time_func(&mut times, || { day04::second(low, high); });
    println!("  Took {}", times.last().unwrap() / 1000000);

    let input = read_file("../input/day05");
    time_func(&mut times, || { day05::first(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);
    time_func(&mut times, || { day05::second(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);

    let input = read_file("../input/day06");
    time_func(&mut times, || { day06::first(&input); });
    println!("  Took {}", times.last().unwrap() / 1000000);

    println!("Total Time {}ms", now.elapsed().unwrap().as_millis());
}
