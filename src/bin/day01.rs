fn main() {
    let input: &str = include_str!("./day01.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut dial: isize = 50;
    let mut count: usize = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let mut distance: isize = line[1..].parse().unwrap();

        // Make everything a right turn
        if direction == 'L' {
            let rotations = distance / 100;
            let offset = (rotations + 1) * 100;
            distance = offset - distance;
        }
        dial += distance;
        dial = dial % 100;

        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let mut dial: isize = 50;
    let mut count: usize = 0;

    for line in input.lines() {
        let c = line.chars().next().unwrap();
        let mut distance: isize = line[1..].parse().unwrap();

        todo!();
    }
    count
}
