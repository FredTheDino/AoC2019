pub fn first(input : &String) {
    let mut program: Vec<i32> = input.split(",")
                                        .map(|s| s.parse().unwrap())
                                        .collect();

    let mut output = Vec::new();
    let input = 1;
    let mut index = 0;
    loop {
        let op_code = program[index] % 100;
        let a_mode: i32 = ((program[index]) / 100) % 10;
        let b_mode: i32 = ((program[index]) / 1000) % 10;
        let c_mode: i32 = ((program[index]) / 10000) % 10;
        match op_code {
            1 => {
                // Add
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = a + b;
                index += 4;
            },
            2 => {
                // Mul
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = a * b;
                index += 4;
            },
            3 => {
                // Input
                let c = program[index + 1] as usize;
                program[c] = input;
                index += 2;
            },
            4 => {
                // Output
                let c = fetch_val(&program, a_mode, index + 1);
                output.push(c);
                index += 2;
            },
            99 => break,
            _ => panic!("Invalid opcode!"),
        }
    }
    println!("05-A: {:?}", output);
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

    let mut output = Vec::new();
    let input = 5;
    let mut index = 0;
    loop {
        let op_code = program[index] % 100;
        let a_mode: i32 = ((program[index]) / 100) % 10;
        let b_mode: i32 = ((program[index]) / 1000) % 10;
        let c_mode: i32 = ((program[index]) / 10000) % 10;
        match op_code {
            1 => {
                // Add
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = a + b;
                index += 4;
            },
            2 => {
                // Mul
                let a = fetch_val(&program, a_mode, index + 1);
                let b = fetch_val(&program, b_mode, index + 2);
                let c = program[index + 3] as usize;
                program[c] = a * b;
                index += 4;
            },
            3 => {
                // Input
                let c = program[index + 1] as usize;
                program[c] = input;
                index += 2;
            },
            4 => {
                // Output
                let c = fetch_val(&program, a_mode, index + 1);
                output.push(c);
                index += 2;
            },
            5 => {
                // JUMP
                let p = fetch_val(&program, a_mode, index + 1);
                let loc = fetch_val(&program, b_mode, index + 2);
                if p != 0 {
                    index = loc as  usize;
                } else {
                    index += 3;
                }
            },
            6 => {
                // JEZ
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
            _ => panic!("Invalid opcode!"),
        }
    }
    println!("05-B: {:?}", output);
}

