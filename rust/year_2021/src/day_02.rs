pub fn part_01(input: Vec<String>) -> i32 {
    let iter = input.iter();

    let mut horizontal = 0;
    let mut depth = 0;

    for line in iter {
        let line_split: Vec<&str> = line.split(" ").collect();
        println!("{}, {}", line_split[0], line_split[1]);
        match line_split[0] {
            "forward" => horizontal += line_split[1].parse::<i32>().unwrap(),
            "down" => depth += line_split[1].parse::<i32>().unwrap(),
            "up" => depth -= line_split[1].parse::<i32>().unwrap(),
            _ => panic!("Bad input"),
        }
    }

    horizontal * depth
}

pub fn part_02(input: Vec<String>) -> i32 {
    let iter = input.iter();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in iter {
        let line_split: Vec<&str> = line.split(" ").collect();
        println!("{}, {}", line_split[0], line_split[1]);
        match line_split[0] {
            "forward" => {
                horizontal += line_split[1].parse::<i32>().unwrap();
                depth += aim * line_split[1].parse::<i32>().unwrap();
            }
            "down" => aim += line_split[1].parse::<i32>().unwrap(),
            "up" => aim -= line_split[1].parse::<i32>().unwrap(),
            _ => panic!("Bad input"),
        }
    }

    horizontal * depth
}
