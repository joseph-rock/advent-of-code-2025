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
            "+" => num_seq.sum(),
            "*" => num_seq.product::<usize>(),
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

    // Gross Vec<char> to String to &str to usize
    let mut num_seq: Vec<usize> = vec![];
    let mut total = 0;
    for line in puzzle {
        let mut line_string: String = line.into_iter().collect();

        // Operator at end of last number in sequence
        if line_string.ends_with('+') || line_string.ends_with('*') {
            let operator = line_string.pop().unwrap();
            let number = &line_string.trim().parse::<usize>().unwrap();
            num_seq.push(*number);

            total += match operator {
                '+' => num_seq.into_iter().sum(),
                '*' => num_seq.into_iter().product::<usize>(),
                _ => panic!("You got some weird operator"),
            };

            num_seq = vec![];
        } else if !line_string.trim().is_empty() {
            let number = &line_string.trim().parse::<usize>().unwrap();
            num_seq.push(*number);
        }
    }

    total
}
