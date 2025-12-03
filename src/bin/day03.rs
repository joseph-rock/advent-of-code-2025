fn main() {
    let input: &str = include_str!("./day03.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let num_vec: Vec<u32> = line.chars().map(|num| num.to_digit(10).unwrap()).collect();

        let mut left: u32 = 0;
        let mut right: u32 = 0;
        for i in 0..num_vec.len() - 1 {
            if let Some(num) = num_vec.get(i) && *num > left {
                left = *num;
                right = 0;
            }
            if let Some(num) = num_vec.get(i+1) && *num > right {
                right = *num;
            }
        }
        total += ((left * 10) + right) as usize;
    }

    total
}
