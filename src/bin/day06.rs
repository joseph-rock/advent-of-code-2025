fn main() {
    let input: &str = include_str!("./day06.txt");
    let pt1 = part_1(input);
    println!("Part 1: {pt1}");
    let pt2 = part_2(input);
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut puzzle: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        for (i, unit) in line.split_whitespace().enumerate() {
            match puzzle.get_mut(i) {
                Some(column) => column.push(unit),
                None => puzzle.push(vec![unit]),
            };
        }
    }

    let mut total = 0;
    for mut seq in puzzle {
        let operator = seq.pop().expect("Should end with a + or *");

        let num_seq = seq.into_iter().map(|num| num.parse::<usize>().unwrap());

        total += match operator {
            "+" => num_seq.fold(0, |acc, num| acc + num),
            "*" => num_seq.fold(1, |acc, num| acc * num),
            _ => panic!("You got some weird operator"),
        }
    }

    total
}

fn part_2(input: &str) -> usize {

    // Rotate puzzle
    let mut puzzle: Vec<Vec<char>> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().rev().enumerate() {
            if y == 0 {
                puzzle.push(vec![character]);
            } else {
                puzzle[x].push(character);
            }
        }
    }

    // Gross String to &str to usize
    let mut num_seq: Vec<usize> = vec![];
    let mut total = 0;
    for line in puzzle {
        let mut line_string: String = line.into_iter().collect();

        if line_string.contains('+') || line_string.contains('*') {
            let operator = line_string.pop().unwrap();
            num_seq.push(*&line_string.trim().parse::<usize>().unwrap());
            total += match operator {
                '+' => num_seq.into_iter().fold(0, |acc, num| acc + num),
                '*' => num_seq.into_iter().fold(1, |acc, num| acc * num),
                _ => panic!("You got some weird operator"),
            };
            num_seq = vec![];
        } else if !line_string.trim().is_empty() {
            num_seq.push(*&line_string.trim().parse::<usize>().unwrap());
        }
    }

    total
}
