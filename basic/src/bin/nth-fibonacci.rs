// Generate the nth Fibonacci number. - From The Rust Book Chapter 3

fn main() {
    println!("Enter the value of n:");
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line!");
    let n: u32 = n.trim().parse().expect("Expected an unsigned number!");

    let mut a = 0;
    let mut b = 1;
    let mut sum = 0;

    for _ in 0..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    println!("The {}th Fibonacci number is {}", n, sum);
}
