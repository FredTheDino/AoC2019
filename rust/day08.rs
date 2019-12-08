
const width: usize = 25;
const height: usize = 6;
const dim: usize = width * height;

pub fn first(input: &String) {
    let mut numbers: Vec<u32> = input.chars()
                           .map(|s| s.to_digit(10).unwrap())
                           .collect();
    let mut best = (1000000, 0, 0);
    let mut index = 0;
    loop {
        let mut layer = (0, 0, 0);
        if index >= numbers.len() { break; }
        for i in index..(index + dim) {
            match numbers[i as usize] {
                | 0 => { layer.0 += 1; },
                | 1 => { layer.1 += 1; },
                | 2 => { layer.2 += 1; },
                | _ => {}
            }
        }
        if best.0 > layer.0 {
            best = layer;
        }
        index += dim;
    }
    println!("{}", best.1 * best.2);
}

pub fn second(input: &String) {
    let mut numbers: Vec<u32> = input.chars()
                           .map(|s| s.to_digit(10).unwrap())
                           .collect();
    let mut layers: Vec<Vec<u32>> = Vec::new();
    const dim: usize = 25 * 6;
    let mut index = 0;
    loop {
        let mut layer = Vec::with_capacity(dim);
        if index >= numbers.len() { break; }
        for i in index..(index + dim) {
            layer.push(numbers[i]);
        }
        layers.push(layer);
        index += dim;
    }

    let mut image: Vec<u32> = Vec::with_capacity(dim);
    for i in 0..dim {
        image.push(2);
    }

    for layer in layers.iter() {
        for i in 0..dim {
            match image[i] {
                | 2 => { image[i] = layer[i]; },
                | _ => {},
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let index = x + y * width;
            if image[index] == 1 {
                print!("O");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
