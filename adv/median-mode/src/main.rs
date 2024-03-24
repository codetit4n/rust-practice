use std::collections::HashMap;

fn main() {
    let mut input = vec![
        9, 10, 12, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24, 25, 25,
    ];

    // let mut input = vec![]; // for testing with empty array

    println!("Input: {:?}", input);

    match median(&mut input) {
        Some(val) => println!("Median: {}", val),
        None => println!("Empty array passed!"),
    }

    let mode_arr = mode(&mut input);
    if mode_arr.len() == 0 {
        println!("No mode found!");
    } else {
        println!("Mode: {:?}", mode_arr);
    }
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

// Finding mode without using iterators
// Considering case where there could be multiple modes or no modes at all
fn mode(inp: &mut Vec<i32>) -> Vec<i32> {
    let mut int_ctr: HashMap<i32, i32> = HashMap::new();
    for val in inp {
        let ctr = int_ctr.entry(*val).or_insert(0);
        *ctr += 1;
    }

    let mut arr = vec![];
    let mut max = 1;
    for (key, val) in int_ctr {
        if val > max {
            arr.clear();
            max = val;
            arr.push(key);
        } else if val == max {
            if val > 1 {
                arr.push(key);
            }
        }
    }

    arr
}
