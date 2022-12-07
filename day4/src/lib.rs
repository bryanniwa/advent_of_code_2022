pub fn part_one(lines: &[String]) {
    let count = lines
        .iter()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(',').expect("No comma");
            let range1 = elf1
                .split('-')
                .map(|x| x.parse::<u32>().expect("Not a number"))
                .collect::<Vec<u32>>();
            let range2 = elf2
                .split('-')
                .map(|x| x.parse::<u32>().expect("Not a number"))
                .collect::<Vec<u32>>();

            (range1[0] <= range2[0] && range1[1] >= range2[1])
                || (range1[0] >= range2[0] && range1[1] <= range2[1])
        })
        .count();

        println!("Part one: {}", count);
}

pub fn part_two(lines: &Vec<String>) {
    println!("Not implemented yet!")
}
