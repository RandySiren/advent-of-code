// Start keeping track of 2 pointers, let the for loop handle one of them and increment result based on if it's increasing.
pub fn part_01(input: Vec<String>) -> i32 {
    let mut iter = input.iter();

    let mut prev: i32 = iter.next().unwrap().parse().unwrap();

    let mut result = 0;

    for current in iter {
        let current_num = current.parse().unwrap();
        if current_num > prev {
            result += 1;
        }
        prev = current_num;
    }
    result
}

// Store 3 values we are using in an array (as opposed to 3 different variables) and increment result based on sum of the subsets if they're increasing.
pub fn part_02(input: Vec<String>) -> i32 {
    let mut iter = input.iter();

    let mut result = 0;

    let mut local_values = [
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    ];

    for _ in 0..input.len() - 3 {
        let sum_1: i32 = local_values.iter().sum();
        local_values = [
            local_values[1],
            local_values[2],
            iter.next().unwrap().parse().unwrap(),
        ];
        let sum_2: i32 = local_values.iter().sum();
        if sum_2 > sum_1 {
            result += 1;
        }
    }
    result
}
