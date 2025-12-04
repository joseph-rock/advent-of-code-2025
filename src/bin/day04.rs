fn main() {
    let input: &str = include_str!("./day04.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    // Collecting then immediately iterating after, but I "need" to reference
    // the grid (to count neighbors). Do better.
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    let paper = '@';

    for (y, line) in grid.clone().into_iter().enumerate() {
        for (x, character) in line.into_iter().enumerate() {
            if character == paper {
                if neighbor_count(&grid, &paper, x, y) < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let paper = '@';
    let removed = 'x';
    let mut paper_removed = true;

    while paper_removed {
        paper_removed = false;
        for (y, line) in grid.clone().into_iter().enumerate() {
            for (x, character) in line.into_iter().enumerate() {
                if character == paper {
                    if neighbor_count(&grid, &paper, x, y) < 4 {
                        grid[y][x] = removed;
                        paper_removed = true;
                    }
                }
            }
        }
    }

    grid.into_iter().fold(0, |sum, line| {
        sum + line.into_iter().filter(|c| c == &removed).count()
    })
}

fn neighbor_count(grid: &Vec<Vec<char>>, paper: &char, x: usize, y: usize) -> usize {
    let neighbor = |xi: usize, yi: usize| -> bool {
        grid.get(yi)
            .is_some_and(|line: &Vec<char>| line.get(xi) == Some(&paper))
    };

    let n_border = y == 0;
    let s_border = y == grid.len() - 1;
    let e_border = x == grid.get(0).expect("0th index exists, trust").len() - 1;
    let w_border = x == 0;

    let n: bool = !n_border && neighbor(x, y - 1);
    let s: bool = !s_border && neighbor(x, y + 1);
    let e: bool = !e_border && neighbor(x + 1, y);
    let w: bool = !w_border && neighbor(x - 1, y);
    let ne: bool = !n_border && !e_border && neighbor(x + 1, y - 1);
    let nw: bool = !n_border && !w_border && neighbor(x - 1, y - 1);
    let se: bool = !s_border && !e_border && neighbor(x + 1, y + 1);
    let sw: bool = !s_border && !w_border && neighbor(x - 1, y + 1);

    usize::from(n)
        + usize::from(s)
        + usize::from(e)
        + usize::from(w)
        + usize::from(ne)
        + usize::from(nw)
        + usize::from(se)
        + usize::from(sw)
}
