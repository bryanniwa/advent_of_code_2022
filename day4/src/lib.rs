pub fn part_one(lines: &[String]) {
    let ranges = get_ranges(lines);
    let count = ranges
        .iter()
        .filter(|pair| {
            let (range1, range2) = pair;
            (range1.min <= range2.min && range1.max >= range2.max)
                || (range1.min >= range2.min && range1.max <= range2.max)
        })
        .count();

    println!("Part one: {}", count);
}

pub fn part_two(lines: &[String]) {
    let ranges = get_ranges(lines);
    let count = ranges
        .iter()
        .filter(|pair| {
            let (range1, range2) = pair;
            (range2.min >= range1.min && range2.min <= range1.max)
                || (range1.min >= range2.min && range1.min <= range2.max)
        })
        .count();

    println!("Part two: {}", count);
}

struct Range {
    min: u32,
    max: u32,
}

fn get_range(line: &str) -> Range {
    let range = line
        .split('-')
        .map(|x| x.parse::<u32>().expect("Not a number"))
        .collect::<Vec<u32>>();

    Range {
        min: range[0],
        max: range[1],
    }
}

fn get_ranges(lines: &[String]) -> Vec<(Range, Range)> {
    lines
        .iter()
        .map(|line| {
            let (elf1, elf2) = line.split_once(',').expect("No comma");
            let range1 = get_range(elf1);
            let range2 = get_range(elf2);

            (range1, range2)
        })
        .collect()
}
