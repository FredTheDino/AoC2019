use std::cmp;

pub fn first(input : &str) {
    let mut wires: Vec<Vec<(char, i32)>> = Vec::new();
    for line in input.split_whitespace() {
        wires.push(line.split(",")
        .map(|s| (s.chars().next().unwrap(), s[1..].parse().unwrap()))
        .collect());
    }

    let mut positions: Vec<Vec<(i32, i32)>> = Vec::new();
    for wire in wires {
        let mut x = 0;
        let mut y = 0;
        let mut points: Vec<(i32, i32)> = Vec::new();
        points.push((x, y));
        for segment in wire {
            match segment.0 {
                'R' => x = x + segment.1,
                'L' => x = x - segment.1,
                'U' => y = y + segment.1,
                'D' => y = y - segment.1,
                _ => panic!("Invalid input"),
            }
            points.push((x, y));
        }
        positions.push(points);
    }
    let (mut intersections, mut steps) = find_intersections(positions);
    intersections.sort();
    steps.sort();
    println!("03-A: Distance {}", intersections[1]);
    println!("03-B: Steps {}", steps[1]);
}

pub fn span_intersect(a1: i32, a2: i32, p1: i32, p2: i32) -> Option<i32> {
    let low;
    let high;
    let p;
    if a1 == a2 {
        low = cmp::min(p1, p2);
        high = cmp::max(p1, p2);
        p = a1;
    } else {
        low = cmp::min(a1, a2);
        high = cmp::max(a1, a2);
        p = p1;
    }
    if low <= p && p <= high { Some(p) }
    else if low <= p && p <= high { Some(p) }
    else { None }
}

pub fn find_intersections(wires: Vec<Vec<(i32, i32)>>) -> (Vec<i32>, Vec<i32>) {
    let mut intersections = Vec::new();
    let mut steps = Vec::new();
    let mut wire_steps_a = 0;
    for (p1, p2) in wires[0].iter().zip(wires[0].iter().skip(1)) {
        let mut wire_steps_b = 0;
        for (q1, q2) in wires[1].iter().zip(wires[1].iter().skip(1)) {
            let x = span_intersect(p1.0, p2.0, q1.0, q2.0);
            let y = span_intersect(p1.1, p2.1, q1.1, q2.1);
            if x.is_some() && y.is_some() {
                steps.push(wire_steps_a + wire_steps_b + (p1 .0 - x.unwrap()).abs() +
                           (q1 .0 - x.unwrap()).abs() +
                           (p1 .1 - y.unwrap()).abs() +
                           (q1 .1 - y.unwrap()).abs());
                intersections.push(x.unwrap().abs() + y.unwrap().abs());
            } 
            wire_steps_b += (q1.0 - q2.0).abs() + (q1.1 - q2.1).abs();
        }
        wire_steps_a += (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
    }
    return (intersections, steps);
}
