use std::collections::HashMap;
use std::cmp::min;
use std::cmp::max;

pub fn first(input : &String) {
    let program: Vec<i64> = input.split(",")
                                 .map(|s| s.parse().unwrap())
                                 .collect();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for i in 0..program.len() {
        let value = program[i as usize];
        memory.insert(i as i64, value);
    }

    let mut screen = HashMap::new();
    screen.reserve(2000);

    let mut min_pos: (i64, i64) = (0, 0);
    let mut max_pos: (i64, i64) = (0, 0);

    reset_int_comp();
    loop {
        let (x, done) = run_program(& mut memory, 0);
        if done { break; }
        let (y, done) = run_program(& mut memory, 0);
        assert!(!done, "Invalid time!");
        let (block, done) = run_program(& mut memory, 0);
        assert!(!done, "Invalid time!");
        
        screen.insert((x, y), block);
        min_pos.0 = min(min_pos.0, x);
        min_pos.1 = min(min_pos.1, y);

        max_pos.0 = max(max_pos.0, x);
        max_pos.1 = max(max_pos.1, y);
    }
    let mut num = 0;
    const PRINT: bool = false;
    if PRINT {
        println!("");
        for y in (min_pos.1..=max_pos.1) {
            println!("");
            for x in (min_pos.0..=max_pos.0) {
                let pos = (x, y);
                let color;
                if screen.contains_key(&pos) {
                    color = *screen.get(&pos).unwrap();
                } else {
                    color = 0;
                }
                match color {
                    0 => print!(" "),
                    1 => print!("W"),
                    2 => print!("B"),
                    3 => print!("H"),
                    4 => print!("O"),
                    _ => panic!("Invalid!"),
                }
                if color == 2 {
                    num += 1;
                }
            }
        }
        println!("");
    } else {
        for (_, value) in screen {
            num += (value == 2) as u32;
        }
    }
    println!("13-A: {}", num);
}

pub fn second(input : &String) {
    let program: Vec<i64> = input.split(",")
                                 .map(|s| s.parse().unwrap())
                                 .collect();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for i in 0..program.len() {
        let value = program[i as usize];
        memory.insert(i as i64, value);
    }
    memory.insert(0, 2);

    let mut score = 0;
    let mut paddle_x: i64 = 0;
    let mut ball_x: i64 = 0;

    reset_int_comp();
    loop {
        let stick = (ball_x - paddle_x).signum();
        let (x, done) = run_program(& mut memory, stick);
        if done { break; }
        let (y, done) = run_program(& mut memory, stick);
        assert!(!done, "Invalid time!");
        let (block, done) = run_program(& mut memory, stick);
        assert!(!done, "Invalid time!");
        
        if x == -1 && y == 0 {
            score = block;
        } else {
            match block {
                3 => paddle_x = x,
                4 => ball_x = x,
                _ => {},
            }
        }
    }

    println!("13-B: {}", score);
}

static mut BASE_PTR: i64 = 0;
fn fetch_val(program: &mut HashMap<i64, i64>, mode: i64, index: i64, addr: bool) -> i64 {
    let at = program[&index];
    if addr {
        if mode == 0 {
            at
        } else if mode == 2 {
            unsafe {
                BASE_PTR + at
            }
        } else {
            panic!("Invalid address mode?");
        }
    } else {
        if mode == 0 {
            *program.entry(at).or_insert(0)
        } else if mode == 1 {
            at
        } else {
            unsafe {
                *program.entry(BASE_PTR + at).or_insert(0)
            }
        }
    }
}

static mut PROGRAM_PTR: i64 = 0;

fn reset_int_comp() {
    unsafe {
        BASE_PTR = 0;
        PROGRAM_PTR = 0;
    }
}

fn run_program(mut program: &mut HashMap<i64, i64>, input: i64) -> (i64, bool) {
    loop {
        let mut index;
        unsafe {
            index = PROGRAM_PTR;
        }

        let op_code = program[&index] % 100;
        let a_mode: i64 = ((program[&index]) / 100) % 10;
        let b_mode: i64 = ((program[&index]) / 1000) % 10;
        let c_mode: i64 = ((program[&index]) / 10000) % 10;
        match op_code {
            1 => {
                // add
                let a = fetch_val(& mut program, a_mode, index + 1, false);
                let b = fetch_val(& mut program, b_mode, index + 2, false);
                let c = fetch_val(& mut program, c_mode, index + 3, true);
                program.insert(c, a + b);
                index += 4;
            },
            2 => {
                // mul
                let a = fetch_val(& mut program, a_mode, index + 1, false);
                let b = fetch_val(& mut program, b_mode, index + 2, false);
                let c = fetch_val(& mut program, c_mode, index + 3, true);
                program.insert(c, a * b);
                index += 4;
            },
            3 => {
                // input
                let c = fetch_val(& mut program, a_mode, index + 1, true);
                program.insert(c, input);
                index += 2;
            },
            4 => {
                // output
                let c = fetch_val(& mut program, a_mode, index + 1, false);
                index += 2;
                unsafe {
                    PROGRAM_PTR = index;
                }
                return (c, false);
            },
            5 => {
                // jump
                let p = fetch_val(& mut program, a_mode, index + 1, false);
                let loc = fetch_val(& mut program, b_mode, index + 2, false);
                if p != 0 {
                    index = loc;
                } else {
                    index += 3;
                }
            },
            6 => {
                // jez
                let p = fetch_val(& mut program, a_mode, index + 1, false);
                let loc = fetch_val(& mut program, b_mode, index + 2, false);
                if p == 0 {
                    index = loc;
                } else {
                    index += 3;
                }
            },
            7 => {
                // <
                let a = fetch_val(& mut program, a_mode, index + 1, false);
                let b = fetch_val(& mut program, b_mode, index + 2, false);
                let c = fetch_val(& mut program, c_mode, index + 3, true);
                program.insert(c, if a < b { 1 } else { 0 });
                index += 4;
            },
            8 => {
                // ==
                let a = fetch_val(& mut program, a_mode, index + 1, false);
                let b = fetch_val(& mut program, b_mode, index + 2, false);
                let c = fetch_val(& mut program, c_mode, index + 3, true);
                program.insert(c, if a == b { 1 } else { 0 });
                index += 4;
            },
            9 => {
                // Update relative
                let a = fetch_val(& mut program, a_mode, index + 1, false);
                unsafe {
                    BASE_PTR += a;
                }
                index += 2;
            }
            99 => break,
            _ => panic!("invalid opcode!"),
        }
        unsafe {
            PROGRAM_PTR = index;
        }
    }
    return (0, true);
}

