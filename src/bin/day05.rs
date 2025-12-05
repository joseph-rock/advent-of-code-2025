fn main() {
    let input: &str = include_str!("./day05.txt");
    let pt1 = part_1(input);
    println!("Part 1: {pt1}");
    // let pt2 = part_2(input);
    // println!("Part 2: {pt2}");
}

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<usize>) {
    let mut ranges: Vec<Range> = vec![];
    let mut numbers: Vec<usize> = vec![];

    for line in input.lines() {
        if line.contains("-") {
            let mut range = line.split("-").map(|num| num.parse::<usize>().unwrap());
            ranges.push(Range {
                min: range.next().unwrap(),
                max: range.next().unwrap(),
            });
        } else {
            if let Ok(number) = line.parse::<usize>() {
                numbers.push(number);
            }
        }
    }

    (ranges, numbers)
}

fn part_1(input: &str) -> usize {
    let mut count = 0;
    let (ranges, numbers) = parse_input(input);

    for number in numbers {
        for range in &ranges {
            if number >= range.min && number <= range.max {
                count += 1;
                break;
            }
        }
    }

    count
}
