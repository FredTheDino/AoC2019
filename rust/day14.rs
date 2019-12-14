use std::collections::HashMap;
use std::cmp::min;

fn parse_ore(rec: &str) -> (i64, &str) {
    let mut it = rec.split_whitespace();
    (it.next().unwrap().parse().unwrap(), it.next().unwrap())
}

type Recipces<'a> = HashMap<&'a str, (i64, Vec<(i64, &'a str)>)>;

pub fn first(input : &String) {
    let recipies: Recipces = input.lines()
        .map(|s| {
            let mut it = s.split("=>");
            let ingrs_str = it.next().unwrap();
            let mut ingrs = Vec::new();
            for i in ingrs_str.split(",") {
                ingrs.push(parse_ore(i));
            }
            let makes = parse_ore(it.next().unwrap());
            (makes.1, (makes.0, ingrs))
        })
        .collect();
    let mut made_stuff: HashMap<&str, i64> = HashMap::new();

    println!("14-A: {}", make_fuel("FUEL", 1, &recipies, &mut made_stuff));
}

fn make_fuel<'a>(name: &'a str, num: i64,
                 recipies: &HashMap<&'a str, (i64, Vec<(i64, &'a str)>)>,
                 mut made_stuff: &mut HashMap<&'a str, i64>) -> i64 {
    if name == "ORE" {
        return num;
    }
    let stored;
    if made_stuff.contains_key(name) { 
        stored = made_stuff.get(name).unwrap().clone();
    } else {
        stored = 0;
    }

    let needed = num - min(num, stored);
    if needed == 0 {
        made_stuff.insert(name, stored - min(num, stored));
        return 0;
    }

    let recipie = recipies.get(name).unwrap();
    let makes = recipie.0;
    let batches = (needed as f64 / makes as f64).ceil() as i64;
    let overflow = (makes - (needed % makes)) % makes;
    made_stuff.insert(name, stored - min(num, stored) + overflow);

    let mut ore_used = 0;
    for (count, ore) in &recipie.1 {
        ore_used += make_fuel(ore, *count * batches, &recipies, &mut made_stuff);
    }

    return ore_used;
}

pub fn second(input : &String) {
    let recipies: Recipces = input.lines()
        .map(|s| {
            let mut it = s.split("=>");
            let ingrs_str = it.next().unwrap();
            let mut ingrs = Vec::new();
            for i in ingrs_str.split(",") {
                ingrs.push(parse_ore(i));
            }
            let makes = parse_ore(it.next().unwrap());
            (makes.1, (makes.0, ingrs))
        })
        .collect();
    let mut low  =  1000000;
    let mut high = 10000000;
    let target = 1000000000000;
    loop {
        let mut made_stuff = HashMap::new();
        let guess = (low + high) / 2;
        let cost = make_fuel("FUEL", guess, &recipies, &mut made_stuff);
        if (high - low) < 2 {
            println!("14-B: {} fuel costing {}", guess, cost);
            break;
        }
        if cost < target {
            low = guess;
        } else if cost > target {
            high = guess;
        }
    }
    // println!("{}", made);
}

// 1663766 LOW, 
