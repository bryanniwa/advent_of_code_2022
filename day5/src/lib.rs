use regex::Regex;

pub fn part_one(lines: &[String]) {
    let mut stacks = get_stacks(lines);
    let instruction_start = stacks.iter().map(|s| s.len()).max().unwrap() + 2;

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines.iter().skip(instruction_start) {
        let captures = re.captures(line).unwrap();
        let from = captures[2].parse::<usize>().unwrap() - 1;
        let to = captures[3].parse::<usize>().unwrap() - 1;
        let value = captures[1].parse::<usize>().unwrap();

        for _ in 0..value {
            let val = stacks[from].pop().unwrap();
            stacks[to].push(val);
        }
    }

    let tops = stacks
        .iter()
        .map(|s| s.last().unwrap())
        .fold(String::new(), |f, s| f + s.to_string().as_str());
    println!("Part one: {:?}", tops);
}

pub fn part_two(lines: &[String]) {
    let mut stacks = get_stacks(lines);
    let instruction_start = stacks.iter().map(|s| s.len()).max().unwrap() + 2;

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in lines.iter().skip(instruction_start) {
        let captures = re.captures(line).unwrap();
        let from = captures[2].parse::<usize>().unwrap() - 1;
        let to = captures[3].parse::<usize>().unwrap() - 1;
        let value = captures[1].parse::<usize>().unwrap();

        let len = stacks[from].len();
        let temp = stacks[from].drain(len - value..).collect::<Vec<_>>();
        stacks[to].extend(temp);
    }

    let tops = stacks
        .iter()
        .map(|s| s.last().unwrap())
        .fold(String::new(), |f, s| f + s.to_string().as_str());
    println!("Part two: {:?}", tops);
}

fn get_stacks(lines: &[String]) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    for line in lines {
        for (stack, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_numeric() {
                return stacks;
            }

            if stacks.len() <= stack {
                stacks.push(Vec::new());
            }

            if c != ' ' {
                stacks[stack].insert(0, c);
            }
        }
    }

    stacks
}
