use std::collections::HashSet;

pub fn part_one(lines: &[String]){
    for line in lines {
        for i in 3..line.len() {
            if line[i-3..=i].chars().collect::<HashSet<char>>().len() == 4 {
                println!("Part one: {}", i+1);
                return;
            }
        }
    }
}

pub fn part_two(lines: &[String]) {
    for line in lines {
        for i in 13..line.len() {
            if line[i-13..=i].chars().collect::<HashSet<char>>().len() == 14 {
                println!("Part two: {}", i+1);
                return;
            }
        }
    }
}
