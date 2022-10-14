use colored::{ColoredString, Colorize};

mod day_01;
mod day_02;

fn main() {
    // Our program needs 3 arguments to be passed in, otherwise error out
    assert_eq!(
        std::env::args().count(),
        3,
        "{}",
        "Invalid arguments, use `cargo run -q input.txt 1.1`"
            .red()
            .bold()
    );

    let input_path = std::env::args().nth(1).unwrap();
    let problem_id = std::env::args().nth(2).unwrap();

    let input: Vec<String> = std::fs::read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|x| String::from(x))
        .collect();

    // Run appropriate question
    match problem_id.as_str() {
        "1.1" => println!("{}", prettify(day_01::part_01(input))),
        "1.2" => println!("{}", prettify(day_01::part_02(input))),
        "2.1" => println!("{}", prettify(day_02::part_01(input))),
        "2.2" => println!("{}", prettify(day_02::part_02(input))),
        _ => panic!("{}", "Not a valid problem".red().bold()),
    };
}

fn prettify(str: i32) -> ColoredString {
    format!("Answer: {}", str.to_string())
        .bright_cyan()
        .bold()
        .on_black()
}
