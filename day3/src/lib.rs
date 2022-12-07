use std::collections::HashSet;

pub fn part_one(lines: &Vec<String>) {
    let mut common_elements = Vec::new();

    for line in lines {
        let section1 = line[..line.len() / 2].chars().collect::<HashSet<char>>();
        let section2 = line[line.len() / 2..].chars().collect::<HashSet<char>>();

        let common = section1.intersection(&section2).copied();
        common_elements.extend(common);
    }
    let value = calculate_value(common_elements);

    println!("Part one: {value}");
}

/* Find the common elements between every three lines in a vector of Strings */
pub fn part_two(lines: &Vec<String>) {
    let mut common_elements = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        let section1 = lines[i].chars().collect();
        let section2 = lines[i + 1].chars().collect();
        let section3 = lines[i + 2].chars().collect();

        common_elements.extend(intersection(vec![section1, section2, section3]));

    }
    let value = calculate_value(common_elements);

    println!("Part two: {value}");
}

pub fn intersection(lines: Vec<Vec<char>>) -> Vec<char> {
    let mut intersect_result: Vec<char> = lines[0].clone();

    for temp_vec in lines {
        let unique_a: HashSet<char> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .copied()
            .collect::<Vec<_>>();
    }
    intersect_result
}

fn calculate_value(items: Vec<char>) -> u32 {
    items
        .into_iter()
        .map(|c| c as u32 - if c.is_lowercase() { 96 } else { 38 })
        .sum()
}
