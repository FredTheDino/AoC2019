pub fn first(input : &String) {
    let module_weights: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let sum : i32 = module_weights.iter()
        .map(fuel_req)
        .sum();
    println!("01-A: {}", sum)
}

fn fuel_req(module: &i32) -> i32 { module / 3 - 2 }

pub fn second(input : &String) {
    let module_weights: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let sum : i32 = module_weights.iter().map(improved_fuel_req).sum();
    println!("01-B: {}", sum)
}

fn improved_fuel_req(module: &i32) -> i32 {
    let mut sum = fuel_req(module);
    let mut last_sum = sum;
    loop {
        let fuel_sum = fuel_req(&last_sum);
        if fuel_sum <= 0 { break sum; }
        sum += fuel_sum;
        last_sum = fuel_sum;
    }
}
