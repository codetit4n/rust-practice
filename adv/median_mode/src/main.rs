use std::collections::HashMap;

fn main() {
    let mut input = vec![
        9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25,
    ];

    // let mut input = vec![]; // for testing with empty array

    println!("Input: {:?}", input);

    //println!("Median: {}", median(&mut input));

    match median(&mut input) {
        Some(val) => println!("Median: {}", val),
        None => println!("Empty array passed!"),
    }

    //@todo
    //println!("Mode: {:?}", mode(&mut input));
}

fn median(inp: &mut Vec<i32>) -> Option<f64> {
    if inp.len() == 0 {
        return None;
    }

    // sorting
    inp.sort();

    // find median
    if inp.len() % 2 == 0 {
        let mid1 = inp.len() / 2;
        let mid2 = inp.len() / 2 - 1;
        Some((inp[mid1] as f64 + inp[mid2] as f64) / 2.0)
    } else {
        let mid = inp.len() / 2;
        Some(inp[mid] as f64)
    }
}

// @todo
// there could be multiple modes or no modes at all
/* fn mode(inp: &mut Vec<i32>) -> Vec<i32> {
    let mut int_ctr: HashMap<i32, i32> = HashMap::new();
    for val in inp {
        let ctr = int_ctr.entry(*val).or_insert(0);
        *ctr += 1;
    }

    for (key, val) in int_ctr {
        println!("{}: {}", key, val);
    }
}*/
