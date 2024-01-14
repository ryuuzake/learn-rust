use std::collections::HashMap;
use std::io;

fn main() {
    println!("Finds median and mode on integer array!");

    loop {
        println!("Please input your array.");

        let mut input = String::new();
        let mut input_array: Vec<i32> = vec![];
        let mut input_map: HashMap<i32, i32> = HashMap::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        for word in input.trim().split_whitespace() {
            let num: i32 = match word.parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            input_array.push(num);

            let count = input_map.entry(num).or_insert(0);
            *count += 1;
        }

        input_array.sort();

        let median_index = input_array.len() / 2;

        let median = if median_index % 2 == 0 {
            (&input_array[median_index] + &input_array[median_index + 1]) / 2
        } else {
            input_array[median_index]
        };

        let mut max_count = 0;
        let mut mode = 0;

        for (key, value) in &input_map {
            if value > &max_count {
                max_count = *value;
                mode = *key;
            }
        }

        println!("median is {median}. mode is {mode}");
    }
}
