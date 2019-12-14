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
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn prev_result(times: &Vec<u64>, end_index: u64) -> u64 {
    return times[times.len() - end_index as usize] / 1000000;
}

fn main() {
    let now = SystemTime::now();
    let mut times = Vec::with_capacity(50);

    if false {
        println!("Day 01:");
        let input = read_file("../input/day01");
        time_func(&mut times, || { day01::first(&input); });
        time_func(&mut times, || { day01::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 02:");
        let input = read_file("../input/day02");
        time_func(&mut times, || { day02::first(&input); });
        time_func(&mut times, || { day02::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        let input = read_file("../input/day03");
        println!("Day 03:");
        time_func(&mut times, || { day03::first(&input); });
        println!("{}ms", prev_result(&times, 1));
        println!("");

        let low = 240298;
        let high = 784956;
        println!("Day 04:");
        time_func(&mut times, || { day04::first(low, high); });
        time_func(&mut times, || { day04::second(low, high); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 05:");
        let input = read_file("../input/day05");
        time_func(&mut times, || { day05::first(&input); });
        time_func(&mut times, || { day05::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 06:");
        let input = read_file("../input/day06");
        time_func(&mut times, || { day06::first(&input); });
        println!("{}ms", prev_result(&times, 1));
        println!("");

        println!("Day 07:");
        let input = read_file("../input/day07");
        time_func(&mut times, || { day07::first(&input); });
        time_func(&mut times, || { day07::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");


        println!("Day 08:");
        let input = read_file("../input/day08");
        time_func(&mut times, || { day08::first(&input); });
        time_func(&mut times, || { day08::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 09:");
        let input = read_file("../input/day09");
        time_func(&mut times, || { day09::first(&input); });
        time_func(&mut times, || { day09::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 10:");
        let input = read_file("../input/day10");
        let mut out = (0, 0);
        time_func(&mut times, || { out = day10::first(&input); });
        time_func(&mut times, || { day10::second(&input, out); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 11:");
        let input = read_file("../input/day11");
        time_func(&mut times, || { day11::first(&input); });
        time_func(&mut times, || { day11::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 12:");
        let input = vec![(-4, -14, 8), (1, -8, 10), (-15, 2, 1), (-17, -17, 16)];
        // let input = vec![(-1,0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
        time_func(&mut times, || { day12::first(&input); });
        time_func(&mut times, || { day12::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");

        println!("Day 13:");
        let input = read_file("../input/day13");
        time_func(&mut times, || { day13::first(&input); });
        time_func(&mut times, || { day13::second(&input); });
        println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
        println!("");
    }

    println!("Day 14:");
    let input = read_file("../input/day14");
    time_func(&mut times, || { day14::first(&input); });
    time_func(&mut times, || { day14::second(&input); });
    println!("{}ms, {}ms", prev_result(&times, 2), prev_result(&times, 1));
    println!("");

    println!("Total Time {}ms", now.elapsed().unwrap().as_millis());
}
