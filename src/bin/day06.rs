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

        let num_seq = seq
            .into_iter()
            .map(|num| num.parse::<usize>().unwrap());

        total += match operator {
            "+" => num_seq.fold(0, |acc, num| acc + num),
            "*" => num_seq.fold(1, |acc, num| acc * num),
            _ => panic!("You got some weird operator"),
        }
    }

    total
}

fn part_2(input: &str) -> usize {
    let mut puzzle: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        for (i, unit) in line.split_whitespace().enumerate() {
            match puzzle.get_mut(i) {
                Some(column) => column.push(unit),
                None => puzzle.push(vec![unit]),
            }
        }
    }

    let mut total = 0;
    for mut seq in puzzle {
        let operator = seq.pop().expect("Should end with a + or *");

        let mut column_numbers: Vec<usize> = vec![];
        for num_string in seq {
            for (i, character) in num_string.chars().rev().enumerate() {
                let number_part: usize = character.to_digit(10).unwrap() as usize;
                match column_numbers.get_mut(i) {
                    Some(number) => *number = *number * 10 + number_part,
                    None => column_numbers.push(number_part),
                }
            }
        }

        dbg!(&column_numbers);
        total += match operator {
            "+" => column_numbers.into_iter().fold(0, |acc, num| acc + num),
            "*" => column_numbers.into_iter().fold(1, |acc, num| acc * num),
            _ => panic!("You got some weird operator"),
        }

    }
    total
}
