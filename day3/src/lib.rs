use std::collections::HashSet;

pub fn part_one(lines: &Vec<String>) {
    let common_elements = find_common(lines);

    let value = calculate_value(common_elements);

    println!("Part one: {value}");
}

fn find_common(lines: &Vec<String>) -> Vec<char> {
    let mut common_elements = Vec::new();

    for line in lines {
        let section1 = line[..line.len() / 2].chars().collect::<HashSet<char>>();
        let section2 = line[line.len() / 2..].chars().collect::<HashSet<char>>();

        let common = section1.intersection(&section2).copied();
        common_elements.extend(common);
    }

    common_elements
}

fn calculate_value(items: Vec<char>) -> u32 {
    items
        .into_iter()
        .map(|c| c as u32 - if c.is_lowercase() { 96 } else { 38 })
        .sum()
}
