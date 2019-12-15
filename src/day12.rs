pub fn first(input: &Vec<(i32, i32, i32)>) {
    let mut planets: Vec<((i32, i32, i32), (i32, i32, i32))> = Vec::new();
    for p in input {
        planets.push((*p, (0, 0, 0)));
    }

    // for i in 0..planets.len() {
    //     let pos = planets.get(i).unwrap().0;
    //     let vel = planets.get(i).unwrap().1;
    //     println!("{} {} {} : {} {} {}", pos.0, pos.1, pos.2, vel.0, vel.1, vel.2);
    // }
    for _ in 0..1000 { simulate(& mut planets); }
    let mut sum = 0;
    for p in planets {
        sum += kinetic_energy(p.0, p.1);
    }
    println!("12-A: {}", sum);
}


fn simulate(planets: &mut Vec<((i32, i32, i32), (i32, i32, i32))>) {
    for i in 0..planets.len() {
        for j in (i+1)..planets.len() {
            let planet_a = planets.get(i).unwrap();
            let a_pos = planet_a.0;
            let mut a_vel = planet_a.1;
            let planet_b = planets.get(j).unwrap();
            let b_pos = planet_b.0;
            let mut b_vel = planet_b.1;

            a_vel.0 -= (a_pos.0 - b_pos.0).signum();
            b_vel.0 -= (b_pos.0 - a_pos.0).signum();
            a_vel.1 -= (a_pos.1 - b_pos.1).signum();
            b_vel.1 -= (b_pos.1 - a_pos.1).signum();
            a_vel.2 -= (a_pos.2 - b_pos.2).signum();
            b_vel.2 -= (b_pos.2 - a_pos.2).signum();

            planets[i] = (a_pos, a_vel);
            planets[j] = (b_pos, b_vel);
        }
    }

    for i in 0..planets.len() {
        let planet = planets.get(i).unwrap();
        let pos = planet.0;
        let vel = planet.1;
        let new_pos = (pos.0 + vel.0, pos.1 + vel.1, pos.2 + vel.2);
        planets[i] = (new_pos, vel);
    }
}

fn get_all_of(planet: ((i32, i32, i32), (i32, i32, i32)), axis: i32) -> (i32, i32) {
    let pos = planet.0;
    let vel = planet.1;
    match axis {
        0 => (pos.0, vel.0),
        1 => (pos.1, vel.1),
        2 => (pos.2, vel.2),
        _ => panic!("Waaa!"),
    }
}

fn kinetic_energy(pos: (i32, i32, i32), vel: (i32, i32, i32)) -> i32 {
    (pos.0.abs() + pos.1.abs() + pos.2.abs()) * 
    (vel.0.abs() + vel.1.abs() + vel.2.abs())
}

fn gdc(a: i64, b: i64) -> i64 {
    if b == 0 { return a; }
    return gdc(b, a % b);
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gdc(a, b);
}

pub fn second(input: &Vec<(i32, i32, i32)>) {
    let mut planets: Vec<((i32, i32, i32), (i32, i32, i32))> = Vec::new();
    for p in input {
        planets.push((*p, (0, 0, 0)));
    }

    let mut coords = [((0, 0), (0, 0), (0, 0), (0, 0)); 3];
    let mut found = [0; 3];

    let mut i = 1;
    simulate(& mut planets);

    for axis in 0..coords.len() {
        let bundle = (get_all_of(planets[0], axis as i32),
                      get_all_of(planets[1], axis as i32),
                      get_all_of(planets[2], axis as i32),
                      get_all_of(planets[3], axis as i32));
        coords[axis] = bundle;
    }
    
    loop { 
        simulate(& mut planets);
        i += 1;
        let mut done = true;
        for axis in 0..=2 {
            if found[axis] != 0 { continue; }
            let bundle = (get_all_of(planets[0], axis as i32),
                          get_all_of(planets[1], axis as i32),
                          get_all_of(planets[2], axis as i32),
                          get_all_of(planets[3], axis as i32));

            if coords[axis] == bundle {
                found[axis] = (i - 1) as i64;
            } else {
                done = false;
            }
        }
        if done {
            break;
        }
    }

    let comp = lcm(lcm(found[0], found[1]), found[2]);
    println!("12-b: {}", comp);
}
