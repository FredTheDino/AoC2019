pub fn first(input : &String) {
    let mut program: Vec<usize> = input.split(",")
                                        .map(|s| s.parse().unwrap())
                                        .collect();

    program[1] = 12;
    program[2] = 2;
    let mut index = 0;
    loop {
        match program[index] {
            1 => {
                let slot = program[index + 3] as usize;
                program[slot] = program[program[index + 1]] + program[program[index + 2]];
            },
            2 => {
                let slot = program[index + 3] as usize;
                program[slot] = program[program[index + 1]] * program[program[index + 2]];
            },
            99 => break,
            _ => panic!("Invalid opcode!"),
        }
        index += 4;
    }
    println!("02-A: {}", program[0]);
}

pub fn second(input : &String) {
    let old_program: Vec<usize> = input.split(",")
                                        .map(|s| s.parse().unwrap())
                                        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = old_program.clone();
            program[1] = noun;
            program[2] = verb;
            let mut index = 0;
            loop {
                match program[index] {
                    1 => {
                        let slot = program[index + 3] as usize;
                        program[slot] = program[program[index + 1]] + program[program[index + 2]];
                    },
                    2 => {
                        let slot = program[index + 3] as usize;
                        program[slot] = program[program[index + 1]] * program[program[index + 2]];
                    },
                    99 => break,
                    _ => panic!("Invalid opcode!"),
                }
                index += 4;
            }

            if program[0] == 19690720 {
                println!("02-B: {}", noun * 100 + verb)
            }
        }
    }
}

