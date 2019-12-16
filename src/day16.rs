use std::collections::HashSet;
use std::collections::HashMap;

pub fn first(input : &String) {
    let mut sequence: Vec<u8> = input.chars()
            .map(|s| s.to_digit(10).unwrap() as u8)
            .collect();
    let mut next = sequence;
    for _ in 0..100 {
        next = fft(&next, 0);
    }
    print!("16-A: ");
    for i in next.iter().take(8) {
        print!("{}", i);
    }
    println!("");
}
// TOO HIGH 79796139

fn fft(sequence: &Vec<u8>, offset: i32) -> Vec<u8> {
    let mut next: Vec<u8> = Vec::with_capacity(sequence.len());
    for i in 0..sequence.len() {
        let index = i + offset as usize;
        let mut scaler = Vec::with_capacity(index * 4);
        for _ in 0..=i { scaler.push( 0); }
        for _ in 0..=i { scaler.push( 1); }
        for _ in 0..=i { scaler.push( 0); }
        for _ in 0..=i { scaler.push(-1); }
        let sum = sequence.iter()
                          .zip(scaler.iter().cycle().skip(1))
                          .fold(0, |sum, zip|
                            sum + (*zip.0 as i32) * (*zip.1 as i32));
        next.push((sum.abs() % 10) as u8);
        println!("loop: {}", i);
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
    let offset = offset as usize;
    let mut list = Vec::new();
    let sequence_length = sequence.len();
    for i in offset..sequence_length {
        list.push(sequence[i]);
    }
    let total_length = sequence_length * 10000 - offset;
    let num_appends = total_length / sequence_length;
    for i in 0..num_appends {
        list.append(&mut sequence.clone());
    }
    println!("len: {}", list.len());

    println!("offset: {}", offset);
    let mut next = list;
    for i in 0..100 {
        next = fft(&next, offset as i32);
        println!("loop: {}", i);
    }
    print!("16-A: ");
    println!("");
}
