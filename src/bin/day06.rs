fn main() {
    let input: &str = include_str!("./day06.txt");
    let pt1 = part_1(input);
    println!("Part 1: {pt1}");
    // let pt2 = part_2(input);
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
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
