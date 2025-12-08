#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

fn distance(left: &Coordinate, right: &Coordinate) -> f64 {
    let xs = left.x as isize - right.x as isize;
    let ys = left.y as isize - right.y as isize;
    let zs = left.z as isize - right.z as isize;
    ((xs * xs) as f64 + (ys * ys) as f64 + (zs * zs) as f64).sqrt()
}

fn main() {
    let input: &str = include_str!("./day08.txt");
    let pt1 = part_1(input);
    println!("Part 1: {pt1}");
    // let pt2 = part_2(input);
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut coordinates: Vec<Coordinate> = vec![];
    for line in input.lines() {
        let mut line_split = line.split(",");
        let x = line_split.next().unwrap().parse::<usize>().unwrap();
        let y = line_split.next().unwrap().parse::<usize>().unwrap();
        let z = line_split.next().unwrap().parse::<usize>().unwrap();

        coordinates.push(Coordinate { x, y, z });
    }

    for i in 0..coordinates.len() - 1 {
        for j in i + 1..coordinates.len() {
            let left = coordinates[i];
            let right = coordinates[j];
            let distance = distance(&left, &right);
            dbg!(left, right, distance);
        }
    }

    0
}
