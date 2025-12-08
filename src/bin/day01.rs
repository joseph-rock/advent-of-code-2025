fn main() {
    let input: &str = include_str!("./day01.txt");
    let pt1 = part_1(input);
    println!("Part 1: {pt1}");
    let pt2 = part_2(input);
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut dial: usize = 50;
    let mut count: usize = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let distance: usize = line[1..].parse().unwrap();

        (dial, _) = turn(direction, dial, distance);

        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> usize {
    let mut dial: usize = 50;
    let mut zeros: usize = 0;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let distance: usize = line[1..].parse().unwrap();

        let (new_dial, turns) = turn(direction, dial, distance);
        dial = new_dial;
        zeros += turns;
    }
    zeros
}

fn turn(direction: char, start: usize, distance: usize) -> (usize, usize) {
    match direction {
        'R' => {
            let dial = (start + distance) % 100;
            let turns = (start + distance) / 100;
            return (dial, turns);
        }
        'L' => {
            let rem = distance % 100;
            if rem > start {
                let dial = 100 - (rem - start);
                // maybe too clever - if we start at 0 then we don't want to add 1
                let turns = distance / 100 + (start != 0) as usize;
                return (dial, turns);
            } else {
                let dial = start - rem;
                let turns = (distance / 100) + (dial == 0) as usize;
                return (dial, turns);
            }
        }
        _ => panic!("bad direction in turn()"),
    }
}
