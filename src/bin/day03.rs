fn main() {
    let input: &str = include_str!("./day03.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let num_vec: Vec<u32> = line.chars().map(|num| num.to_digit(10).unwrap()).collect();

        let mut left: u32 = 0;
        let mut right: u32 = 0;
        for i in 0..num_vec.len() - 1 {
            if let Some(num) = num_vec.get(i)
                && *num > left
            {
                left = *num;
                right = 0;
            }
            if let Some(num) = num_vec.get(i + 1)
                && *num > right
            {
                right = *num;
            }
        }
        total += left * 10 + right;
    }

    total
}

// lmao
fn part_2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let num_vec: Vec<usize> = line
            .chars()
            .map(|num| num.to_digit(10).unwrap() as usize)
            .collect();

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        let mut e = 0;
        let mut f = 0;
        let mut g = 0;
        let mut h = 0;
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut l = 0;

        for index in 0..num_vec.len() - 11 {
            if let Some(num) = num_vec.get(index)
                && *num > a
            {
                a = *num;
                b = 0;
                c = 0;
                d = 0;
                e = 0;
                f = 0;
                g = 0;
                h = 0;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 1)
                && *num > b
            {
                b = *num;
                c = 0;
                d = 0;
                e = 0;
                f = 0;
                g = 0;
                h = 0;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 2)
                && *num > c
            {
                c = *num;
                d = 0;
                e = 0;
                f = 0;
                g = 0;
                h = 0;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 3)
                && *num > d
            {
                d = *num;
                e = 0;
                f = 0;
                g = 0;
                h = 0;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 4)
                && *num > e
            {
                e = *num;
                f = 0;
                g = 0;
                h = 0;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 5)
                && *num > f
            {
                f = *num;
                g = 0;
                h = 0;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 6)
                && *num > g
            {
                g = *num;
                h = 0;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 7)
                && *num > h
            {
                h = *num;
                i = 0;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 8)
                && *num > i
            {
                i = *num;
                j = 0;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 9)
                && *num > j
            {
                j = *num;
                k = 0;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 10)
                && *num > k
            {
                k = *num;
                l = 0;
            }
            if let Some(num) = num_vec.get(index + 11)
                && *num > l
            {
                l = *num;
            }
        }
        total += a * 100_000_000_000
            + b * 10_000_000_000
            + c * 1_000_000_000
            + d * 100_000_000
            + e * 10_000_000
            + f * 1_000_000
            + g * 100_000
            + h * 10_000
            + i * 1_000
            + j * 100
            + k * 10
            + l;
    }

    total
}
