use std::io;

fn main() {
    println!("N Fibonacci");

    loop {
        println!("Please input n.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let n_fibonacci = n_fibonacci(input);

        println!("The nth fibonacci series: {n_fibonacci}");
    }
}

fn n_fibonacci(n: u32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut num = 0;

        for _ in 0..(n - 2) {
            num = a + b;
            (a, b) = (b, a + b);
        }

        num
    }
}
