pub fn first(input : &String) {
    let mut program: Vec<i32> = input.split(",")
                                        .map(|s| s.parse().unwrap())
                                        .collect();

    let programs = [
        program.clone(),
        program.clone(),
        program.clone(),
        program.clone(),
        program.clone(),
    ];

    let mut maximum = 0;
    for a in 0..=4 {
        for b in 0..=4 {
            if b == a { continue; }
            for c in 0..=4 {
                if c == a || c == b { continue; }
                for d in 0..=4 {
                    if d == a || d == b || d == c { continue; }
                    for e in 0..=4 {
                        if e == a || e == b || e == c || e == d { continue; }
                        let out = run_program(& mut program.clone(), a, 0);
                        let out = run_program(& mut program.clone(), b, out);
                        let out = run_program(& mut program.clone(), c, out);
                        let out = run_program(& mut program.clone(), d, out);
                        let out = run_program(& mut program.clone(), e, out);
                        if out > maximum {
                            maximum = out;
                        }
                    }
                }
            }
        }
    }
    println!("07-A: Largest: {}", maximum);
}

fn fetch_val(program: &Vec<i32>, mode: i32, index: usize) -> i32 {
    let at = program[index];
    if mode == 0 {
        program[at as usize]
    } else {
        at
    }
}

pub fn second(input : &String) {
    let mut program: Vec<i32> = input.split(",")
                                        .map(|s| s.parse().unwrap())
                                        .collect();

    let mut maximum = 0;
    // TODO(ed): This can be made into a "combinations".
    for a in 5..=9 {
        for b in 5..=9 {
            if b == a { continue; }
            for c in 5..=9 {
                if c == a || c == b { continue; }
                for d in 5..=9 {
                    if d == a || d == b || d == c { continue; }
                    for e in 5..=9 {
                        if e == a || e == b || e == c || e == d { continue; }
                        let mut programs = [
                            (program.clone(), 0),
                            (program.clone(), 0),
                            (program.clone(), 0),
                            (program.clone(), 0),
                            (program.clone(), 0),
                        ];
                        let phase = [a, b, c, d, e];
                        let mut out = 0;
                        for (mut prog, phase) in programs.iter_mut().zip(phase.iter()) {
                                out = run_program_better(&mut prog.1,
                                                         &mut prog.0,
                                                         *phase, out).0;
                        }

                        let mut done = false;
                        loop {
                            for prog in programs.iter_mut() {
                                let tmp = run_program_better(&mut prog.1,
                                                             &mut prog.0,
                                                             out, 0);
                                if !tmp.1 {
                                    out = tmp.0;
                                } else {
                                    done = tmp.1;
                                }
                            }
                            if done { break; }
                        }
                        if out > maximum {
                            maximum = out;
                        }
                    }
                }
            }
        }
    }
    println!("07-B: Largest: {}", maximum);
}

fn run_program(program: &mut Vec<i32>, phase: i32, signal: i32) -> i32 {
    let mut selector = 0;
    let mut output = -1;
    let mut index = 0;
    loop {
        let op_code = program[index] % 100;
        let a_mode: i32 = ((program[index]) / 100) % 10;
        let b_mode: i32 = ((program[index]) / 1000) % 10;
        let c_mode: i32 = ((program[index]) / 10000) % 10;
        match op_code {
            1 => {
                // add
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = a + b;
                index += 4;
            },
            2 => {
                // mul
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = a * b;
                index += 4;
            },
            3 => {
                // input
                let c = program[index + 1] as usize;
                if selector == 0 {
                    program[c] = phase;
                } else {
                    program[c] = signal;
                }
                selector += 1;
                index += 2;
            },
            4 => {
                // output
                let c = fetch_val(&program, a_mode, index + 1);
                return c;
                index += 2;
            },
            5 => {
                // jump
                let p = fetch_val(&program, a_mode, index + 1);
                let loc = fetch_val(&program, b_mode, index + 2);
                if p != 0 {
                    index = loc as  usize;
                } else {
                    index += 3;
                }
            },
            6 => {
                // jez
                let p = fetch_val(&program, a_mode, index + 1);
                let loc = fetch_val(&program, b_mode, index + 2);
                if p == 0 {
                    index = loc as usize;
                } else {
                    index += 3;
                }
            },
            7 => {
                // <
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = if a < b { 1 } else { 0 };
                index += 4;
            },
            8 => {
                // ==
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = if a == b { 1 } else { 0 };
                index += 4;
            },
            99 => break,
            _ => panic!("invalid opcode!"),
        }
    }
    return output;
}

fn run_program_better(index: &mut usize, program: &mut Vec<i32>, phase: i32, signal: i32) -> (i32, bool) {
    let mut selector = 0;
    loop {
        let op_code = program[*index] % 100;
        let a_mode: i32 = ((program[*index]) / 100) % 10;
        let b_mode: i32 = ((program[*index]) / 1000) % 10;
        let c_mode: i32 = ((program[*index]) / 10000) % 10;
        match op_code {
            1 => {
                // add
                let a = fetch_val(&program, a_mode, *index + 1);
                let b = fetch_val(&program, b_mode, *index + 2);
                let c = program[*index + 3] as usize;
                program[c] = a + b;
                *index += 4;
            },
            2 => {
                // mul
                let a = fetch_val(&program, a_mode, *index + 1);
                let b = fetch_val(&program, b_mode, *index + 2);
                let c = program[*index + 3] as usize;
                program[c] = a * b;
                *index += 4;
            },
            3 => {
                // input
                let c = program[*index + 1] as usize;
                if selector == 0 {
                    program[c] = phase;
                } else {
                    program[c] = signal;
                }
                selector += 1;
                *index += 2;
            },
            4 => {
                // output
                let c = fetch_val(&program, a_mode, *index + 1);
                *index += 2;
                return (c, false);
            },
            5 => {
                // jump
                let p = fetch_val(&program, a_mode, *index + 1);
                let loc = fetch_val(&program, b_mode, *index + 2);
                if p != 0 {
                    *index = loc as  usize;
                } else {
                    *index += 3;
                }
            },
            6 => {
                // jez
                let p = fetch_val(&program, a_mode, *index + 1);
                let loc = fetch_val(&program, b_mode, *index + 2);
                if p == 0 {
                    *index = loc as usize;
                } else {
                    *index += 3;
                }
            },
            7 => {
                // <
                let a = fetch_val(&program, a_mode, *index + 1);
                let b = fetch_val(&program, b_mode, *index + 2);
                let c = program[*index + 3] as usize;
                program[c] = if a < b { 1 } else { 0 };
                *index += 4;
            },
            8 => {
                // ==
                let a = fetch_val(&program, a_mode, *index + 1);
                let b = fetch_val(&program, b_mode, *index + 2);
                let c = program[*index + 3] as usize;
                program[c] = if a == b { 1 } else { 0 };
                *index += 4;
            },
            99 => break,
            _ => panic!("invalid opcode!"),
        }
    }
    return (0, true);
}
