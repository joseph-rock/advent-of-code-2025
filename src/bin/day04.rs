fn main() {
    let input: &str = include_str!("./day04.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let paper = '@';

    for (y, line) in grid.clone().into_iter().enumerate() {
        for (x, character) in line.into_iter().enumerate() {
            if character == paper {
                if neighbor_count(&grid, x, y) < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn neighbor_count(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let paper = '@';

    let n_border = y == 0;
    let s_border = y == grid.len() - 1;
    let e_border = x == grid.get(0).expect("0th index exists, trust").len() - 1;
    let w_border = x == 0;

    let n: bool = !n_border && grid.get(y-1).is_some_and(|line| line.get(x) == Some(&paper));
    let s: bool = !s_border && grid.get(y+1).is_some_and(|line| line.get(x) == Some(&paper));
    let e: bool = !e_border && grid.get(y).is_some_and(|line| line.get(x+1) == Some(&paper));
    let w: bool = !w_border && grid.get(y).is_some_and(|line| line.get(x-1) == Some(&paper));
    let ne: bool = !n_border && !e_border && grid.get(y-1).is_some_and(|line| line.get(x+1) == Some(&paper));
    let nw: bool = !n_border && !w_border && grid.get(y-1).is_some_and(|line| line.get(x-1) == Some(&paper));
    let se: bool = !s_border && !e_border && grid.get(y+1).is_some_and(|line| line.get(x+1) == Some(&paper));
    let sw: bool = !s_border && !w_border && grid.get(y+1).is_some_and(|line| line.get(x-1) == Some(&paper));

    usize::from(n)
        + usize::from(s)
        + usize::from(e)
        + usize::from(w)
        + usize::from(ne)
        + usize::from(nw)
        + usize::from(se)
        + usize::from(sw)
}
