use std::collections::HashMap;
use std::cmp::min;
use std::cmp::max;

fn rotate_ccw(dir: (i32, i32)) -> (i32, i32) {
    return (dir.1, -dir.0)
}

fn rotate_cw(dir: (i32, i32)) -> (i32, i32) {
    return (-dir.1, dir.0)
}

fn move_pos(pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    return (pos.0 + dir.0, pos.1 + dir.1);
}

pub fn first(input : &String) {
    let program: Vec<i64> = input.split(",")
                                 .map(|s| s.parse().unwrap())
                                 .collect();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for i in 0..program.len() {
        let value = program[i as usize];
        memory.insert(i as i64, value);
    }

    let mut hull: HashMap<(i32, i32), i64> = HashMap::new();
    hull.reserve(2000);
    let mut pos: (i32, i32) = (0, 0);
    let mut dir: (i32, i32) = (0, 1);

    let mut min_pos: (i32, i32) = (0, 0);
    let mut max_pos: (i32, i32) = (0, 0);

    reset_int_comp();
    loop {
        let mut color = 0;
        if hull.contains_key(&pos) {
            color = *hull.get(&pos).unwrap();
        }
        let result = run_program(& mut memory, color);
        let done = result.1;
        hull.insert(pos, result.0[0]);
        if result.0[1] == 1 {
            dir = rotate_cw(dir);
        } else {
            dir = rotate_ccw(dir);
        }

        pos = move_pos(pos, dir);

        min_pos.0 = min(min_pos.0, pos.0);
        min_pos.1 = min(min_pos.1, pos.1);

        max_pos.0 = max(max_pos.0, pos.0);
        max_pos.1 = max(max_pos.1, pos.1);
        if done { break }
    }
    // println!("");
    // for y in (min_pos.1..=max_pos.1).rev() {
    //     println!("");
    //     for x in (min_pos.0..=max_pos.0).rev() {
    //         let pos = (x, y);
    //         let color;
    //         if hull.contains_key(&pos) {
    //             color = *hull.get(&pos).unwrap();
    //         } else {
    //             color = 0;
    //         }
    //         if color == 1 {
    //             print!("\u{25A0}");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    // }
    println!("");
    print!("11-A: {}", hull.keys().len());
    println!("");
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

    let mut hull: HashMap<(i32, i32), i64> = HashMap::new();
    hull.reserve(2000);
    let mut pos: (i32, i32) = (0, 0);
    let mut dir: (i32, i32) = (0, 1);

    let mut min_pos: (i32, i32) = (0, 0);
    let mut max_pos: (i32, i32) = (0, 0);

    reset_int_comp();
    loop {
        let mut color = 1;
        if hull.contains_key(&pos) {
            color = *hull.get(&pos).unwrap();
        }
        let result = run_program(& mut memory, color);
        let done = result.1;
        if result.0.len() == 2 {
            hull.insert(pos, result.0[0]);
            if result.0[1] == 1 {
                dir = rotate_cw(dir);
            } else {
                dir = rotate_ccw(dir);
            }
            pos = move_pos(pos, dir);

            min_pos.0 = min(min_pos.0, pos.0);
            min_pos.1 = min(min_pos.1, pos.1);

            max_pos.0 = max(max_pos.0, pos.0);
            max_pos.1 = max(max_pos.1, pos.1);
        }
        if done { break }
    }

    print!("11-B:");
    for y in (min_pos.1..=max_pos.1).rev() {
        println!("");
        for x in (min_pos.0..=max_pos.0).rev() {
            let pos = (x, y);
            let color;
            if hull.contains_key(&pos) {
                color = *hull.get(&pos).unwrap();
            } else {
                color = 0;
            }
            if color == 1 {
                print!("\u{25A0}");
            } else {
                print!(" ");
            }
        }
    }
    println!("");
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

fn run_program(mut program: &mut HashMap<i64, i64>, input: i64) -> (Vec<i64>, bool) {
    let mut read_input = false;
    let mut output = Vec::new();
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
                if read_input {
                    return (output, false);
                }
                read_input = true;
                let c = fetch_val(& mut program, a_mode, index + 1, true);
                program.insert(c, input);
                index += 2;
            },
            4 => {
                // output
                let c = fetch_val(& mut program, a_mode, index + 1, false);
                output.push(c);
                index += 2;
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
    return (output, true);
}

