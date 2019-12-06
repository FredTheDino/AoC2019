use std::collections::HashMap;

pub fn first(input : &String) {
    let orbits: Vec<(&str, &str)> = input.split_whitespace()
        .map(|s| {
            let mut it = s.split(")");
            let a = it.next().unwrap();
            let b = it.next().unwrap();
            (a, b)
        })
    .collect();
    let mut orbit_map = HashMap::new();
    for orbit in orbits {
        if !orbit_map.contains_key(orbit.0) {
            orbit_map.insert(orbit.0, Vec::new());
        }
        orbit_map.get_mut(orbit.0).unwrap().push(orbit.1);
    }

    let sum = sum_depth("COM", &orbit_map, 0);
    println!("06-A: {}", sum);

    let mut san_path: Vec<&str> = Vec::new();
    find("SAN", "COM", &orbit_map, & mut san_path);

    let mut you_path: Vec<&str> = Vec::new();
    find("YOU", "COM", &orbit_map, & mut you_path);

    let mut shared = 0;
    for (a, b) in san_path.iter().zip(you_path.iter()) {
        if a != b { break; }
        shared += 1;
    }
    println!("06-B: {}", san_path.len() + you_path.len() - 2 * shared);
}

fn sum_depth(current: &str, orbit_map: &HashMap<&str, Vec<&str>>, depth: u32) -> u32 {
    if !orbit_map.contains_key(current) { return depth; }
    let sum : u32 = orbit_map.get(current).unwrap().iter()
             .map(|s| sum_depth(s, &orbit_map, depth + 1))
             .sum();
    sum + depth
}

fn find<'a>(target: &str, current: &str, orbit_map: &HashMap<&str, Vec<&'a str>>, path: &mut Vec<&'a str>) -> bool {
    if target == current { return true; }
    if !orbit_map.contains_key(current) { return false; }
    for orbit in orbit_map.get(current).unwrap() {
        path.push(orbit);
        if find(target, orbit, &orbit_map, path) {
            return true;
        }
        path.pop();
    }
    return false;
}
