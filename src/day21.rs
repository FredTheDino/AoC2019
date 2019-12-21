use std::collections::HashMap;

pub fn first(input: &String) {
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
    let string = "\
OR C T
NOT C J
OR J T
OR T J
AND C T
AND B T
AND A T
NOT T T
AND D J
AND T J
WALK
".to_string();
    loop {
        let result = run_program(&mut memory, &string);
        if result.1 { break; }
        if result.0 < 255 {
            output.push((result.0 as u8) as char);
        } else {
            println!("DID IT: {}", result.0);
        }
    }
    let output: String = output.into_iter().collect();
    println!("{}", output);
}

pub fn second(input: &String) {
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
    let string = "\
OR C T
NOT C J
OR J T
OR T J
AND C T
AND B T
AND A T
NOT T T
AND D T
NOT J J
OR F J
OR I J
AND E J
OR H J
AND T J
RUN
".to_string();
    loop {
        let result = run_program(&mut memory, &string);
        if result.1 { break; }
        if result.0 < 255 {
            output.push((result.0 as u8) as char);
        } else {
            println!("DID IT: {}", result.0);
        }
    }
    let output: String = output.into_iter().collect();
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
        INPUT_PTR = 0;
    }
}

static mut INPUT_PTR: usize = 0;
fn run_program(mut program: &mut HashMap<i64, i64>, string: &String) -> (i64, bool) {
    let mut it = string.chars().skip(unsafe {INPUT_PTR});
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
                    
                let i = match it.next() {
                    Some(x) => x,
                    _ => return (-2, true),
                };
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
