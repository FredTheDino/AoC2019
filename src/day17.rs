use std::collections::HashMap;
use std::collections::HashSet;

pub fn first(input : &String) -> (HashSet<(i32, i32)>, (i32, i32)) {
    let program: Vec<i64> = input.split(",")
                                 .map(|s| s.parse().unwrap())
                                 .collect();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for i in 0..program.len() {
        let value = program[i as usize];
        memory.insert(i as i64, value);
    }
    reset_int_comp();
    let mut output: Vec<char> = Vec::new();
    loop {
        let result = run_program(& mut memory);
        if result.1 { break; }
        output.push((result.0 as u8) as char);
    }

    let row_length = output.iter().position(|s| *s == '\n').unwrap() as i32;
    let row_length = row_length + 1;
    let mut sum = 0;
    let mut map = HashSet::new();
    let mut pos = (0, 0);
    for x in 0..(row_length - 1) {
        for y in 0..(row_length - 1) {
            let h = '#';
            let curr = output[to_linear(x + 0, y + 0, row_length)];
            if curr == '.' { continue; }
            if curr == '^' { pos = (x, y); }
            map.insert((x, y));
            if x <= 1 || x <= (row_length - 1) { continue; }
            if y <= 1 || y <= (row_length - 1) { continue; }
            if output[to_linear(x - 1, y + 0, row_length)] != h { continue; }
            if output[to_linear(x + 1, y + 0, row_length)] != h { continue; }
            if output[to_linear(x + 0, y + 1, row_length)] != h { continue; }
            if output[to_linear(x + 0, y - 1, row_length)] != h { continue; }
            output[to_linear(x, y, row_length)] = 'X';
            sum += x * y;
        }
    }
    let output: String = output.into_iter().collect();
    println!("{}", output);
    // Too low 4790
    println!("17-A: {}", sum);
    return (map, pos);
}

fn to_linear(x: i32, y: i32, dim: i32) -> usize {
    return (y + x * dim) as usize;
}

fn rotate_left(dir: (i32, i32)) -> (i32, i32) {
    return (-dir.1, dir.0)
}

fn rotate_right(dir: (i32, i32)) -> (i32, i32) {
    return (dir.1, -dir.0)
}

fn move_add(pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    return (pos.0 + dir.0, pos.1 + dir.1);
}

use std::io;

pub fn second(input: &String, map: &HashSet<(i32, i32)>, pos: (i32, i32)) {
    let program: Vec<i64> = input.split(",")
                                 .map(|s| s.parse().unwrap())
                                 .collect();
    let path: Vec<char> = Vec::new();
    let mut curr = pos;
    let mut dir  = (-1, 0);
    let mut steps = 0;
    // let mut visited = HashSet::new();
    let mut sequence = "".to_string();
    /*
    loop {
        let forward = move_add(curr, dir);
        let left    = move_add(curr, rotate_left(dir));
        let right   = move_add(curr, rotate_right(dir));

        visited.insert(curr);
        if false {
            for x in -2..53 {
                for y in -2..53 {
                    if (x, y) == left {
                        if map.contains(&left) {
                            print!("L");
                        } else {
                            print!("l");
                        }
                    } else if (x, y) == right {
                        if map.contains(&right) {
                            print!("R");
                        } else {
                            print!("r");
                        }
                    } else if (x, y) == forward {
                        if map.contains(&forward) {
                            print!("F");
                        } else {
                            print!("f");
                        }
                    } else if visited.contains(&(x, y)) {
                        print!("X");
                    } else if map.contains(&(x, y)) {
                        print!("O");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            println!("{} {}", curr.0, curr.1);
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text);
        }

        if map.contains(&forward) {
            steps += 1;
            curr = forward;
            continue
        }
        if steps != 0 {
            sequence.push_str(&steps.to_string());
            sequence.push(',');
        }
        if map.contains(&left) {
            sequence.push('L');
            dir = rotate_left(dir);
        } else if map.contains(&right) {
            sequence.push('R');
            dir = rotate_right(dir);
        } else {
            break;
        }
        sequence.push(',');
        steps = 0;
    }
    sequence.pop();
    println!("{}", sequence);
    */

    let mut memory: HashMap<i64, i64> = HashMap::new();
    for i in 0..program.len() {
        let value = program[i as usize];
        memory.insert(i as i64, value);
    }
    memory.insert(0, 2);
    let mut output = "".to_string();
    reset_int_comp();
    loop {
        let result = run_program(& mut memory);
        if result.0 < 256 {
            output.push((result.0 as u8) as char);
        } else {
            println!("out: {}", result.0);
        }
        if result.1 { break; }
    }
    println!("{}", output);
}

// INT-code comp
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

static mut INPUT_PTR: usize = 0;
fn run_program(mut program: &mut HashMap<i64, i64>) -> (i64, bool) {
    let string: String = "A,B,A,C,B,C,B,A,C,B\nL,10,L,6,R,10\nR,6,R,8,R,8,L,6,R,8\nL,10,R,8,R,8,L,10\nn\n".to_string();
    let mut it;
    unsafe {
        it = string.chars().skip(INPUT_PTR);
    }
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
                    
                let i = (it.next().unwrap()).clone();
                program.insert(c, i as i64);
                unsafe {
                    INPUT_PTR += 1;
                }
                index += 2;
            },
            4 => {
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
