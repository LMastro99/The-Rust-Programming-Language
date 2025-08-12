use std::io;

fn main() {
    println!("Enter a number to find the nth Fibonacci number: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please type a number!");

    let fibonacci = fibonacci(n);
    if n == 11 || n == 12 || n == 13 {
        println!("The {n}th Fibonacci number is: {fibonacci}");
    } else if n % 10 == 1 {
        println!("The {n}st Fibonacci number is: {fibonacci}");
    } else if n % 10 == 2 {
        println!("The {n}nd Fibonacci number is: {fibonacci}");
    } else if n % 10 == 3 {
        println!("The {n}rd Fibonacci number is: {fibonacci}");
    } else {
        println!("The {n}th Fibonacci number is: {fibonacci}");
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    b
}
