pub fn part_one(lines: &[String]) {
    let ranges = get_ranges(lines);
    let count = ranges.iter().filter(|pair| {
        let (range1, range2) = pair;
        let (min1, max1) = (range1[0], range1[1]);
        let (min2, max2) = (range2[0], range2[1]);
        (min1 <= min2 && max1 >= max2) || (min1 >= min2 && max1 <= max2)
    }).count();

    println!("Part one: {}", count);
}

pub fn part_two(lines: &[String]) {
    let ranges = get_ranges(lines);
    let count = ranges.iter().filter(|pair| {
        let (range1, range2) = pair;
        let (min1, max1) = (range1[0], range1[1]);
        let (min2, max2) = (range2[0], range2[1]);
        (min2 >= min1 && min2 <= max1) || (min1 >= min2 && min1 <= max2)
    }).count();

    println!("Part two: {}", count);
}

fn get_ranges(lines: &[String]) -> Vec<(Vec<u32>, Vec<u32>)> {
    lines
        .iter()
        .map(|line| {
            let (elf1, elf2) = line.split_once(',').expect("No comma");
            let range1 = elf1
                .split('-')
                .map(|x| x.parse::<u32>().expect("Not a number"))
                .collect::<Vec<u32>>();
            let range2 = elf2
                .split('-')
                .map(|x| x.parse::<u32>().expect("Not a number"))
                .collect::<Vec<u32>>();

            (range1, range2)
        })
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>()
}
