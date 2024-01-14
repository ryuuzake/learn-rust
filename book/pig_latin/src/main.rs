use std::io;

fn main() {
    println!("Pig Latin Translator!");

    loop {
        println!("Please input your word.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("Translate into");

        for word in input.trim().split_whitespace() {
            let translate_word = pig_latin_translate(word);

            print!("{translate_word} ");
        }

        println!("");
    }
}

fn pig_latin_translate(word: &str) -> String {
    let char = &word[..1];

    match char {
        "a" => format!("{}-hay", &word),
        "i" => format!("{}-hay", &word),
        "u" => format!("{}-hay", &word),
        "e" => format!("{}-hay", &word),
        "o" => format!("{}-hay", &word),
        other => format!("{}-{}ay", &word[1..], other),
    }
}
