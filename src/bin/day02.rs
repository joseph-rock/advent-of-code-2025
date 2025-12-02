use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("./day02.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    let ranges: Vec<&str> = input
        .strip_suffix("\n")
        .expect("Line ends with single \n char")
        .split(",")
        .collect();

    for r in ranges {
        let range: Vec<usize> = r
            .split("-")
            .map(|number| {
                number
                    .parse::<usize>()
                    .expect("Input promised to be a number")
            })
            .collect();
        let min: usize = range[0];
        let max: usize = range[1];

        for number in min..max + 1 {
            let num_string = number.to_string();
            if num_string.len() % 2 == 0 {
                let mid = num_string.len() / 2;
                let left = &num_string[..mid];
                let right = &num_string[mid..];

                if left == right {
                    total += number;
                }
            }
        }
    }
    total
}

fn part_2(input: &str) -> usize {
    let mut total = 0;
    let ranges: Vec<&str> = input
        .strip_suffix("\n")
        .expect("Line ends with single \n char")
        .split(",")
        .collect();

    for r in ranges {
        let range: Vec<usize> = r
            .split("-")
            .map(|number| {
                number
                    .parse::<usize>()
                    .expect("Input promised to be a number")
            })
            .collect();
        let min: usize = range[0];
        let max: usize = range[1];

        for number in min..max + 1 {
            if solve(number) {
                total += number;
            }
        }
    }

    total
}

fn solve(number: usize) -> bool {
    let num_string: Vec<char> = number.to_string().chars().collect();
    let mid = num_string.len() / 2;

    for i in 1..mid + 1 {
        let foo = num_string.rchunks(i).all(|chunk| *chunk == num_string[..i]);
        if foo {
            return true;
        }
    }
    false
}
