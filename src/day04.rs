
pub fn first(low: i32, high: i32) {
    let mut num = 0;
    let mut i = low;
    loop {
        num += is_valid(i) as i32;
        if i / 100000 < (i / 10000) % 10 { i += 10000; }
        i += 1;
        if i > high { break; }
    }
    println!("04-A: {}", num);
}

fn is_valid(code: i32) -> bool {
    let mut digits = code;
    let mut same = false;
    loop {
        let last = digits % 10;
        let next = (digits / 10) % 10;
        same |= last == next;
        if last < next {
            return false;
        }
        digits /= 10;

        if digits < 10 { break }
    }
    return same;
}

pub fn second(low: i32, high: i32) {
    let mut num = 0;
    let mut i = low;
    loop {
        num += is_more_valid(i) as i32;
        if i / 100000 < (i / 10000) % 10 { i += 10000; }
        i += 1;
        if i > high { break; }
    }
    println!("04-B: {}", num);
}

fn is_more_valid(code: i32) -> bool {
    let mut digits = code;
    let mut num = [0; 10];
    loop {
        let last  = (digits / 1) % 10;
        let next  = (digits / 10) % 10;
        num[last as usize] = num[last as usize] + 1;
        if last < next {
            return false;
        }
        if digits < 10 { break }
        digits /= 10;
    }

    for n in num.iter() {
        if *n == 2 { 
            return true;
        }
    }
    return false;
}
