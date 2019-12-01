pub fn first(input : &String) {
    let module_weights: Vec<i32> = input.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();

    let mut sum = 0;
    for x in module_weights {
        sum += fule_req(&x)
    }
    println!("DAY01-F: {}", sum)
}

fn fule_req(module: &i32) -> i32 { module / 3 - 2 }

pub fn second(input : &String) {
    let module_weights: Vec<i32> = input.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();

    let mut sum = 0;
    for x in module_weights {
        sum += imporved_fule_req(&x)
    }
    println!("DAY02-F: {}", sum)
}

fn imporved_fule_req(module: &i32) -> i32 {
    let mut sum = fule_req(module);
    let mut last_sum = sum;
    loop {
        let fule_sum = fule_req(&last_sum);
        if fule_sum <= 0 { break; }
        sum += fule_sum;
        last_sum = fule_sum;
    }
    sum
}
