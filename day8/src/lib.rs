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

fn scan(forest: &mut [Vec<Tree>]) -> u32 {
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

    count
}

pub fn part_one(lines: &[String]) {
    let mut forest = get_forest(lines);
    let count = scan(&mut forest);

    for row in forest {
        for tree in row {
            match tree.seen {
                true => print!("T"),
                false => print!("F"),
            }
        }
        println!();
    }

    println!("Part one: {}", count);
}

pub fn part_two(lines: &[String]) {
    println!("Not done yet");
}
