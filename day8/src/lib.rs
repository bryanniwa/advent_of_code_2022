#[derive(Debug)]
struct Tree {
    height: i32,
    seen: bool,
}

fn get_forest(lines: &[String]) -> Vec<Vec<Tree>> {
    let mut forest = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            let tree = Tree {
                height: c.to_digit(10).unwrap() as i32,
                seen: false,
            };
            row.push(tree);
        }
        forest.push(row);
    }
    forest
}

pub fn part_one(lines: &[String]) {
    let mut forest = get_forest(lines);
    let mut count = 0;
    let mut col_tracker = vec![-1; forest[0].len()];

    for row in forest.iter_mut() {
        let mut row_max = -1;
        for (x, tree) in row.iter_mut().enumerate() {
            if tree.height > row_max {
                row_max = tree.height;
                if !tree.seen {
                    tree.seen = true;
                    count += 1;
                }
            }

            if tree.height > col_tracker[x] {
                col_tracker[x] = tree.height;
                if !tree.seen {
                    tree.seen = true;
                    count += 1;
                }
            }
        }
    }

    col_tracker = vec![-1; forest[0].len()];
    for row in forest.iter_mut().rev() {
        let mut row_max = -1;
        for (x, tree) in row.iter_mut().rev().enumerate() {
            if tree.height > row_max {
                row_max = tree.height;
                if !tree.seen {
                    tree.seen = true;
                    count += 1;
                }
            }

            if tree.height > col_tracker[x] {
                col_tracker[x] = tree.height;
                if !tree.seen {
                    tree.seen = true;
                    count += 1;
                }
            }
        }
    }

    println!("Part one: {}", count);
}

fn scan(y: usize, x: usize, forest: &mut [Vec<Tree>]) -> u32 {
    let height = forest[y][x].height;

    let mut up = 0;
    for i in (0..=y-1).rev() {
        up += 1;
        if height <= forest[i][x].height {
            break;
        }
    }

    let mut down = 0;
    for i in y+1..forest.len() {
        down += 1;
        if height <= forest[i][x].height {
            break;
        }
    }

    let mut left = 0;
    for i in (0..=x-1).rev() {
        left += 1;
        if height <= forest[y][i].height {
            break;
        }
    };

    let mut right = 0;
    for i in x+1..forest[0].len() {
        right += 1;
        if height <= forest[y][i].height {
            break;
        }
    }

    up * down * left * right

}

pub fn part_two(lines: &[String]) {
    let mut forest = get_forest(lines);
    let mut max = 0;

    for x in 1..forest[0].len()-1 {
        for y in 1..forest.len()-1 {
            let count = scan(x, y, &mut forest);
            if count > max {
                max = count;
            }
        }
    }

    println!("Part two: {}", max);
}
