mod problem_01;

fn main() {
    // Our program needs 3 arguments to be passed in, otherwise error out
    assert_eq!(
        std::env::args().count(),
        3,
        "Invalid arguments, use `cargo run -q input.txt 1.1`"
    );

    let input_path = std::env::args().nth(1).unwrap();
    let problem_id = std::env::args().nth(2).unwrap();

    let input: Vec<String> = std::fs::read_to_string(input_path)
        .expect("Unable to read input file.")
        .lines()
        .map(|x| String::from(x))
        .collect();

    // Run appropriate question
    match problem_id.as_str() {
        "1.1" => problem_01::part_01(input),
        "1.2" => problem_01::part_02(input),
        _ => println!("Not a valid problem."),
    };
}
