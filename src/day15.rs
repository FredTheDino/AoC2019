use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::cmp::min;
use std::cmp::max;

fn move_pos(pos: (i64, i64), dir: i64) -> (i64, i64) {
    match dir {
        1 => (pos.0 + 0, pos.1 + 1),
        2 => (pos.0 + 0, pos.1 - 1),
        3 => (pos.0 + 1, pos.1 + 0),
        4 => (pos.0 - 1, pos.1 + 0),
        _ => panic!("Invalid input"),
    }
}

fn find_unexplored(pos: (i64, i64), map: &HashMap<(i64, i64), i64>) -> i64 {
    let mut visited = HashSet::new();
    visited.insert(pos);
    let mut to_visit = VecDeque::new();
    for dir in [1, 2, 3, 4].iter() {
        to_visit.push_back((move_pos(pos, *dir), *dir));
    }
    loop {
        if to_visit.is_empty() { return 0; }
        let (current, choice) = to_visit.pop_front().unwrap();
        if !map.contains_key(&current) { return choice; }
        if visited.contains(&current) { continue; }
        if *map.get(&current).unwrap() == 0 { continue; }
        visited.insert(current);
        for dir in [1, 2, 3, 4].iter() {
            to_visit.push_back((move_pos(current, *dir), choice));
        }
    }
}

fn find_shortest_path(from: (i64, i64), to: (i64, i64), 
                      map: &HashMap<(i64, i64), i64>) -> i64 {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((from, 0));
    loop {
        if to_visit.is_empty() { return 0; }
        let (current, distance) = to_visit.pop_front().unwrap();
        if current == to { return distance; }
        if visited.contains(&current) { continue; }
        if !map.contains_key(&current) { continue; }
        if *map.get(&current).unwrap() == 0 { continue; }
        visited.insert(current);
        for dir in [1, 2, 3, 4].iter() {
            to_visit.push_back((move_pos(current, *dir), distance + 1));
        }
    }
}

fn find_maximum_distance(from: (i64, i64), map: &HashMap<(i64, i64), i64>) -> i64 {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((from, 0));
    let mut dist = 0;
    loop {
        if to_visit.is_empty() { break; }
        let (current, distance) = to_visit.pop_front().unwrap();
        if visited.contains(&current) { continue; }
        if !map.contains_key(&current) { continue; }
        if *map.get(&current).unwrap() == 0 { continue; }
        visited.insert(current);
        dist = max(dist, distance);
        for dir in [1, 2, 3, 4].iter() {
            to_visit.push_back((move_pos(current, *dir), distance + 1));
        }
    }
    return dist;
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

    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut min_pos = (0, 0);
    let mut max_pos = (0, 0);
    map.insert(pos, 1);
    let mut direction = 0;
    let mut target = (0, 0);
    loop {
        direction = find_unexplored(pos, &map);
        if direction == 0 { break; }
        let next_pos = move_pos(pos, direction);
        let result = run_program(& mut memory, direction as i64);
        map.insert(next_pos, result.0);
        if result.0 != 0 {
            pos = next_pos;
        }

        if (result.0 == 2) {
            target = next_pos;
        }

        min_pos.0 = min(min_pos.0, next_pos.0);
        min_pos.1 = min(min_pos.1, next_pos.1);

        max_pos.0 = max(max_pos.0, next_pos.0);
        max_pos.1 = max(max_pos.1, next_pos.1);

        if false {
        println!("");
        for y in min_pos.1..=max_pos.1 {
            for x in min_pos.0..=max_pos.0 {
                let map_pos = (x, y);
                if map_pos == pos {
                    print!("O");
                    continue;
                }
                if map_pos == (0, 0) {
                    print!("H");
                    continue;
                }
                let sym;
                if map.contains_key(&map_pos) {
                    sym = match *map.get(&map_pos).unwrap() {
                        0 => '#',
                        1 => '.',
                        2 => '2',
                        _ => panic!("Invalid dict!"),
                    };
                } else {
                    sym = ' ';
                }
                print!("{}", sym);
            }
            println!("");
        }
        }
    }
    let distance = find_shortest_path((0, 0), target, &map);
    println!("15-A: {}", distance);
    let distance = find_maximum_distance(target, &map);
    println!("15-B: {}", distance);
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
    println!("no...");
    panic!("Should not finish!");
    return (0, true);
}

