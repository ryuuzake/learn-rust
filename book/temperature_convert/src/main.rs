use std::io;

fn main() {
    println!("F to C Temperature Converter");

    loop {
        println!("Please input fahrenheit temperature.");

        let mut input_temperature = String::new();

        io::stdin()
            .read_line(&mut input_temperature)
            .expect("Failed to read line");

        let input_temperature: f64 = match input_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("You entered: {input_temperature}");

        let converted_temperature = fahrenheit_to_celsius(input_temperature);

        println!("{input_temperature} F = {converted_temperature} C");
    }
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}
