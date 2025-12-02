fn main() {
    let input: &str = include_str!("./day01.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut dial: isize = 50;
    let mut count: usize = 0;
    
    for line in input.lines() {
        let c = line.chars().next().unwrap();
        let mut distance: isize = line[1..].parse().unwrap();

        if c == 'L' {
            distance = distance * -1;
        }

        dial += distance;

        while dial < 0 {
            dial = 100 + dial;
        }
        while dial > 99 {
            dial = dial - 100;
        }

        dbg!(dial);

        if dial == 0 {
            count += 1;
        }

    }
    count
}
