use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::cmp::Reverse;

pub fn first(input: &String) {
    let mut map: Vec<char> = input.chars().collect();
    let dim = (map.iter().position(|s| *s == '\n').unwrap() as i32) + 1;
    let start = map.iter().position(|s| *s == '@').unwrap() as i32;
    // let output: String = map.into_iter().collect();
    let (deps, distances, quadrant) = find_letters(to_pos(start, dim), dim, &mut map);
    // list_all_combinations(&deps);
    let path = shortest_path_v1(&deps, &distances);
    println!("{}", path);
    let path = shortest_path_v2(&deps, &distances, &quadrant);
    println!("{}", path);
}

fn to_linear(x: i32, y: i32, dim: i32) -> usize {
    return (y + x * dim) as usize;
}

fn to_pos(p: i32, dim: i32) -> (i32, i32) {
    let x = (p / dim) as i32;
    let y = (p % dim) as i32;
    return (x, y);
}

fn char_to_bit(c: char) -> u32 {
    const OFFSET: u8 = 'A' as u8;
    return 1 << ((c.to_uppercase().next().unwrap() as u8) - OFFSET);
}

fn shortest_path_v1(deps: &HashMap<char, HashSet<char>>, distances: &HashMap<(char, char), i32>) -> i32 {
    let all_visited = {
        let mut visited = 0;
        for c in deps.keys() {
            visited |= char_to_bit(*c);
        }
        visited
    };

    let mut to_visit = BinaryHeap::new();
    let mut visited_nodes: HashSet<(char, u32)> = HashSet::new();
    to_visit.push((Reverse(0), 0, '@', 0));
    loop {
        if to_visit.is_empty() { return -1; }
        let (_, dist, at, visited) = to_visit.pop().unwrap();
        if visited == all_visited { return dist; }

        if visited_nodes.contains(&(at, visited)) { continue; }
        visited_nodes.insert((at, visited));
        'outer: for (c, reqs) in deps {
            if visited & char_to_bit(*c) != 0 { continue; }
            // Make sure all deps are meet
            for r in reqs {
                if visited & char_to_bit(*r) == 0 { continue 'outer; }
            }
            let dist = distances.get(&(at, *c)).unwrap() + dist;
            to_visit.push((Reverse(dist), dist, *c, visited | char_to_bit(*c)));
        }
    }
}

fn shortest_path_v2(deps: &HashMap<char, HashSet<char>>, distances: &HashMap<(char, char), i32>, quadrant: &HashMap<char, char>) -> i32 {
    let all_visited = {
        let mut visited = 0;
        for c in deps.keys() {
            visited |= char_to_bit(*c);
        }
        visited
    };

    let mut to_visit = BinaryHeap::new();
    let mut visited_nodes: HashSet<((char, char, char, char), u32)> = HashSet::new();
    to_visit.push((Reverse(0), 0, ('1', '2', '3', '4'), 0));
    loop {
        if to_visit.is_empty() { return -1; }
        let (_, dist, at, visited) = to_visit.pop().unwrap();
        if visited == all_visited { return dist; }

        if visited_nodes.contains(&(at, visited)) { continue; }
        visited_nodes.insert((at, visited));
        'outer: for (c, reqs) in deps {
            if visited & char_to_bit(*c) != 0 { continue; }
            // Make sure all deps are meet
            for r in reqs {
                if visited & char_to_bit(*r) == 0 { continue 'outer; }
            }
            let (dist, at) = match quadrant.get(c).unwrap() {
                '1' => { (distances.get(&(at.0, *c)).unwrap() + dist,
                          (*c, at.1, at.2, at.3)) },
                '2' => { (distances.get(&(at.1, *c)).unwrap() + dist,
                          (at.0, *c, at.2, at.3)) },
                '3' => { (distances.get(&(at.2, *c)).unwrap() + dist,
                          (at.0, at.1, *c, at.3)) },
                '4' => { (distances.get(&(at.3, *c)).unwrap() + dist,
                          (at.0, at.1, at.2, *c)) },
                _ => panic!("NOOO!"),
            };
            to_visit.push((Reverse(dist), dist, at, visited | char_to_bit(*c)));
        }
    }
}

fn find_letters(pos: (i32, i32), dim: i32,
                mut map: &mut Vec<char>) -> (HashMap<char, HashSet<char>>,
                                             HashMap<(char, char), i32>,
                                             HashMap<char, char>) {

    fn find_letters_helper(pos: (i32, i32), mut visited: &mut HashSet<(i32, i32)>,
                           mut map: &mut Vec<char>,
                           mut dep: &mut HashMap<char, HashSet<char>>,
                           mut positions: &mut HashMap<char, (i32, i32)>,
                           mut doors: &mut HashSet<char>,
                           mut quadrant: &mut HashMap<char, char>,
                           dim: i32) {
        if visited.contains(&pos) { return; }
        visited.insert(pos);
        let curr = map[to_linear(pos.0, pos.1, dim)];
        if curr == '#' { return; }
        if curr.is_lowercase() { 
            positions.insert(curr, pos);
            dep.insert(curr, doors.clone());
            if pos.0 < (dim / 2) {
                if pos.1 < (dim / 2) {
                    quadrant.insert(curr, '3');
                } else {
                    quadrant.insert(curr, '4');
                }
            } else {
                if pos.1 < (dim / 2) {
                    quadrant.insert(curr, '2');
                } else {
                    quadrant.insert(curr, '1');
                }
            }
        }
        if curr.is_lowercase() || curr.is_uppercase() { doors.insert(curr); }
        for delta in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
            find_letters_helper(new_pos, 
                                &mut visited, 
                                &mut map, 
                                &mut dep, 
                                &mut positions,
                                &mut doors, 
                                &mut quadrant,
                                dim);
        }
        if curr.is_lowercase() || curr.is_uppercase() { doors.remove(&curr); }
    }

    let mut visited = HashSet::new();
    let mut dep = HashMap::new();
    let mut positions = HashMap::new();
    let mut doors = HashSet::new();
    let mut quadrant = HashMap::new();
    find_letters_helper(pos,
                        &mut visited,
                        &mut map,
                        &mut dep,
                        &mut positions,
                        &mut doors,
                        &mut quadrant,
                        dim);

    fn find_distance_helper(a: (i32, i32), b: (i32, i32),
                            map: &Vec<char>, dim: i32) -> i32 {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back((a, 0));
        loop {
            if to_visit.is_empty() { return -1; }
            let (pos, distance) = to_visit.pop_front().unwrap();
            if map[to_linear(pos.0, pos.1, dim)] == '#' { continue; }
            if pos == b { return distance; }
            if visited.contains(&pos) { continue; }
            visited.insert(pos);
            for delta in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
                to_visit.push_back((new_pos, distance + 1));
            }
        }
    }

    let mut distances = HashMap::new();
    for (i, a) in dep.keys().enumerate() {
        for b in dep.keys().skip(i+1) {
            let distance = find_distance_helper(positions[a], positions[b], &map, dim);
            distances.insert((*a, *b), distance);
            distances.insert((*b, *a), distance);
        }
        let distance = find_distance_helper(pos, positions[a], &map, dim);
        distances.insert(('@', *a), distance);
        let distance = find_distance_helper((pos.0 + 1, pos.1 + 1), positions[a], &map, dim);
        distances.insert(('1', *a), distance);
        let distance = find_distance_helper((pos.0 + 1, pos.1 - 1), positions[a], &map, dim);
        distances.insert(('2', *a), distance);
        let distance = find_distance_helper((pos.0 - 1, pos.1 - 1), positions[a], &map, dim);
        distances.insert(('3', *a), distance);
        let distance = find_distance_helper((pos.0 - 1, pos.1 + 1), positions[a], &map, dim);
        distances.insert(('4', *a), distance);
    }
    return (dep, distances, quadrant);
}
