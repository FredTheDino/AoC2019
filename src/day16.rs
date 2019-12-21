
pub fn first(input : &String) {
    let sequence: Vec<u8> = input.chars()
            .map(|s| s.to_digit(10).unwrap() as u8)
            .collect();
    let mut next = sequence;
    for _ in 0..100 {
        fft(&mut next);
    }
    print!("16-A: ");
    for i in next.iter().take(8) {
        print!("{}", i);
    }
    println!("");
}
// TOO HIGH 79796139

fn fft(sequence: &mut Vec<u8>) {
    for i in 0..=(sequence.len() / 2) {
        let mut slot = i;
        let mut sum = 0;
        // Something wrong
        loop {
            for _ in 0..=i {
                if slot >= sequence.len() { break; }
                sum += unsafe { *sequence.get_unchecked(slot) } as i32;
                slot += 1;
            }
            slot += i + 1;
            for _ in 0..=i {
                if slot >= sequence.len() { break; }
                sum -= unsafe { *sequence.get_unchecked(slot) } as i32;
                slot += 1;
            }
            slot += i + 1;
            if slot >= sequence.len() { break; }
        }
        sequence[i] = (sum.abs() % 10) as u8;
    }
    let offset = sequence.len() / 2 + 1;
    ffft(&mut sequence[offset..], offset as i32);
}

fn ffft(sequence: &mut [u8], offset: i32) {
    assert!(sequence.len() < offset as usize);
    let mut total_sum: u64 = sequence.iter().fold(0u64, |s, a| s + (*a as u64));
    for i in 0..sequence.len() {
        let digit = (total_sum % 10) as u8;
        total_sum -= sequence[i as usize] as u64;
        sequence[i as usize] = digit;
    }
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
    let num_appends = total_length / sequence_length;
    for _ in 0..num_appends {
        list.append(&mut sequence.clone());
    }

    let mut next = list;
    for _ in 0..100 {
        ffft(&mut next, offset as i32);
    }
    // 42572042 too high
    print!("16-B: ");
    for i in next.iter().take(8) {
        print!("{}", i);
    }
    println!("");
    println!("");
}
