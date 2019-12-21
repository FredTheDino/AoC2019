use std::collections::HashSet;
use std::collections::HashMap;

pub fn first(input : &String) -> (i32, i32) {
    let map: Vec<char> = input.chars()
            .filter(|s| *s != '\n')
            .collect();

    let dim: i32 = input.chars().position(|s| s == '\n').unwrap() as i32;

    let mut maximum = 0;
    let mut point = (0, 0);
    for y in 0..dim {
        for x in 0..dim {
            if map[to_linear(x, y, dim)] == '.' { continue; }
            let mut current: HashSet<(i32, i32)> = HashSet::new();
            for px in 0..dim {
                for py in 0..dim {
                    if map[to_linear(px, py, dim)] == '.' { continue; }
                    if px == x && py == y { continue; }
                    let slope = slope_from((x - px, y - py));
                    assert!(slope != (0, 0));
                    if !current.contains(&slope) {
                        current.insert(slope);
                        let num_asteroids = current.len();
                        if maximum < num_asteroids {
                            maximum = num_asteroids;
                            point = (x, y);
                        }
                    }
                }
            }
        }
    }
    println!("10-A: ({} {}), {}", point.0, point.1, maximum);
    return point;
}

fn gdc(a: i32, b: i32) -> i32 {
    if b == 0 { return a; }
    return gdc(b, a % b);
}

fn slope_from(delta: (i32, i32)) -> (i32, i32) {
    if delta.0 == 0 || delta.1 == 0 {
        return (delta.0.signum(), delta.1.signum());
    }
    let common = gdc(delta.0, delta.1).abs();
    return (delta.0 / common, delta.1 / common);
}

fn to_linear(x: i32, y: i32, dim: i32) -> usize {
    return (y + x * dim) as usize;
}

fn length_sq(delta: (i32, i32)) -> i32 {
    delta.0 * delta.0 + delta.1 * delta.1
}


pub fn second(input : &String, point: (i32, i32)) {
    let mut map: Vec<char> = input.chars()
            .filter(|s| *s != '\n')
            .collect();

    let dim: i32 = input.chars().position(|s| s == '\n').unwrap() as i32;


    // Maps slopes to positions:
    // let mut found: Vec<HashMap<(i32, i32), i32>>;
    // found = vec![HashMap::new(); (dim * dim) as usize];
    let mut hitlist: HashMap<(i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    for px in 0..dim {
        for py in 0..dim {
            if map[to_linear(px, py, dim)] == '.' { continue; }
            if px == point.0 && py == point.1 { continue; }
            let delta = (point.0 - px, point.1 - py);
            let slope = slope_from(delta);
            assert!(slope != (0, 0));
            if !hitlist.contains_key(&slope) {
                hitlist.insert(slope, Vec::new());
            }
            // Negative so the closest one is destroyed first.
            hitlist.get_mut(&slope).unwrap().push((-length_sq(delta), px, py));
        }
    }
    map[to_linear(point.0, point.1, dim)] = 'X';

    for hits in hitlist.values_mut() {
        hits.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    }

    let mut keys: Vec<_> = hitlist.keys().map(|k| *k).collect();

    keys.sort_unstable_by_key(|a| ((((-a.1) as f64).atan2(a.0 as f64) * 10000.0) as u64));
    let mut i = 0;
    let mut destoryed = 1;
    loop {
        let popped = hitlist.get_mut(&keys[i % keys.len()]).unwrap().pop();
        if popped.is_some() {
            let (_, x, y) = popped.unwrap();
            map[to_linear(x, y, dim)] = (destoryed % 10).to_string().chars()
                                                                    .next()
                                                                    .unwrap();
            // vizualise(&map, dim);
            destoryed += 1;
        }
        if destoryed == 201 {
            let unwrapped = popped.unwrap();
            println!("10-B: {} ({}, {})", unwrapped.2 * 100 + unwrapped.1, unwrapped.1, unwrapped.2);
            // 414 too low, 512 too low, 1518 WRONG, 607 wrong
            return;
        }
        i += 1;
    }
}

#[allow(unused_imports)]
use std::io;
#[allow(dead_code)]
fn vizualise(map: &Vec<char>, dim: i32) {
    let mut input_text = String::new();
    let _ = io::stdin().read_line(&mut input_text);
    for px in 0..dim {
        println!("");
        for py in 0..dim {
            print!("{}", map[to_linear(px, py, dim)]);
        }
    }
    println!("");
}
