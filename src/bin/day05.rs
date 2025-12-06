#[derive(Debug, Copy, Clone)]
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

fn main() {
    let input: &str = include_str!("./day05.txt");
    let pt1 = part_1(input);
    println!("Part 1: {pt1}");
    let pt2 = part_2(input);
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let (ranges, numbers) = parse_input(input);

    numbers
        .into_iter()
        .filter(|number| {
            ranges
                .clone()
                .into_iter()
                .any(|range| *number >= range.min && *number <= range.max)
        })
        .count()
}

fn part_2(input: &str) -> usize {
    let (mut ranges, _) = parse_input(input);
    let mut condensed_ranges = ranges.clone();

    while let Some(range) = ranges.pop() {
        condensed_ranges = condense(range, condensed_ranges);
    }

    condensed_ranges
        .into_iter()
        .fold(0, |acc, range: Range| acc + range.max + 1 - range.min)
}

fn condense(range: Range, ranges: Vec<Range>) -> Vec<Range> {
    let mut return_ranges: Vec<Range> = vec![];
    let mut range_clone = range.clone();

    for check_range in ranges {
        if overlap(&range_clone, &check_range) {
            range_clone.min = lowest(range_clone.min, check_range.min);
            range_clone.max = highest(range_clone.max, check_range.max);
        } else {
            return_ranges.push(check_range);
        }
    }

    return_ranges.push(range_clone);
    return_ranges
}

fn overlap(left: &Range, right: &Range) -> bool {
    !(left.max < right.min || left.min > right.max)
}

fn highest(left: usize, right: usize) -> usize {
    if left >= right {
        return left;
    }
    right
}

fn lowest(left: usize, right: usize) -> usize {
    if left <= right {
        return left;
    }
    right
}
