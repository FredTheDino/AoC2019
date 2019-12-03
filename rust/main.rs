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

fn main() {
    let now = SystemTime::now();
    let mut times = Vec::with_capacity(50);

    let input = read_file("../input/day01");
    time_func(&mut times, || { day01::first(&input); });
    time_func(&mut times, || { day01::second(&input); });

    let input = read_file("../input/day02");
    time_func(&mut times, || { day02::first(&input); });
    time_func(&mut times, || { day02::second(&input); });

    let input = read_file("../input/day03");
    time_func(&mut times, || { day03::first(&input); });
    time_func(&mut times, || { day03::second(&input); });

    println!("Total Time {}ms", now.elapsed().unwrap().as_millis());
}
