use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("./day07.txt");
    let pt1 = part_1(input);
    println!("Part 1: {pt1}");
    let pt2 = part_2(input);
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let start_char = 'S';
    let splitter = '^';
    let mut indexes: HashSet<usize> = HashSet::new();
    let mut total = 0;

    for (line_num, line) in input.lines().enumerate() {
        for (i, character) in line.chars().enumerate() {
            if line_num == 0 && character == start_char {
                indexes.insert(i);
                continue;
            }

            if indexes.contains(&i) && character == splitter {
                indexes.remove(&i);
                indexes.insert(i - 1);
                indexes.insert(i + 1);
                total += 1;
            }
        }
    }
    total
}

fn part_2(input: &str) -> usize {
    let start_char = 'S';
    let splitter = '^';
    let mut indexes: HashSet<usize> = HashSet::new();
    let mut total = 0;

    for (line_num, line) in input.lines().enumerate() {
        let mut split = false;
        for (i, character) in line.chars().enumerate() {
            if line_num == 0 && character == start_char {
                indexes.insert(i);
                continue;
            }

            if indexes.contains(&i) && character == splitter {
                indexes.remove(&i);
                indexes.insert(i - 1);
                indexes.insert(i + 1);
                split = true;
            }
        }
        if split {
            total += indexes.len();
        }
    }
    total
}
