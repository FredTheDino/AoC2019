use std::collections::HashSet;
use std::collections::HashMap;

pub fn first(input : &String) {
    let mut sequence: Vec<u8> = input.chars()
            .map(|s| s.to_digit(10).unwrap() as u8)
            .collect();
    let mut next = sequence;
    for _ in 0..100 {
        next = fft(&next);
    }
    print!("16-A: ");
    for i in next.iter().take(8) {
        print!("{}", i);
    }
    println!("");
}
// TOO HIGH 79796139

fn fft(sequence: &Vec<u8>) -> Vec<u8> {
    let mut next: Vec<u8> = Vec::with_capacity(sequence.len());
    for i in 0..sequence.len() {
        let index = i as usize;
        let mut slot = i;
        let mut sum = 0;
        // Something wrong
        loop {
            for _ in 0..=index {
                if slot >= sequence.len() { break; }
                sum += sequence[slot] as i32;
                slot += 1;
            }
            slot += (index + 1);
            for _ in 0..=index {
                if slot >= sequence.len() { break; }
                sum -= sequence[slot] as i32;
                slot += 1;
            }
            slot += (index + 1);
            if slot >= sequence.len() { break; }
        }
        next.push((sum.abs() % 10) as u8);
    }
    return next;
}

fn ffft(sequence: &Vec<u8>, offset: i32) -> Vec<u8> {
    assert!(sequence.len() < offset as usize);
    let mut total_sum: u64 = sequence.iter().fold(0u64, |s, a| s + (*a as u64));
    let mut next: Vec<u8> = Vec::with_capacity(sequence.len());
    for i in 0..sequence.len() {
        next.push((total_sum % 10) as u8);
        total_sum -= sequence[i as usize] as u64;
    }
    return next;
}

pub fn second(input : &String) {
    let sequence: Vec<u8> = input.chars()
         .map(|s| s.to_digit(10).unwrap() as u8)
         .collect();
    let offset: u32 = input.chars()
                           .take(7)
                           .collect::<String>()
                           .parse().unwrap();
    // 68640286
    let offset = (offset) as usize;
    let mut list = Vec::new();
    let sequence_length = sequence.len();
    for i in (offset % sequence_length)..sequence_length {
        list.push(sequence[i]);
    }
    let total_length = sequence_length * 10000 - offset;
    let num_appends = (total_length / sequence_length);
    for i in 0..num_appends {
        list.append(&mut sequence.clone());
    }

    let mut next = list;
    for i in 0..100 {
        next = ffft(&next, offset as i32);
    }
    // 42572042 too high
    print!("16-B: ");
    for i in next.iter().take(8) {
        print!("{}", i);
    }
    println!("");
    println!("");
}
