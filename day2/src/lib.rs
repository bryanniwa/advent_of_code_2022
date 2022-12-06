use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            RPS::Rock => match other {
                RPS::Rock => Some(Ordering::Equal),
                RPS::Paper => Some(Ordering::Less),
                RPS::Scissors => Some(Ordering::Greater),
            },
            RPS::Paper => match other {
                RPS::Rock => Some(Ordering::Greater),
                RPS::Paper => Some(Ordering::Equal),
                RPS::Scissors => Some(Ordering::Less),
            },
            RPS::Scissors => match other {
                RPS::Rock => Some(Ordering::Less),
                RPS::Paper => Some(Ordering::Greater),
                RPS::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl RPS {
    fn from_letter(letter: char) -> Option<RPS> {
        match letter {
            'A' | 'X' => Some(RPS::Rock),
            'B' | 'Y' => Some(RPS::Paper),
            'C' | 'Z' => Some(RPS::Scissors),
            _ => None,
        }
    }

    fn lose(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock
        }
    }
    
    fn win(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper
        }
    }

    fn value(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

fn calculate_score(elf: RPS, mine: RPS) -> u32 {
    mine.value()
        + if mine > elf {
            6
        } else if mine == elf {
            3
        } else {
            0
        }
}

pub fn part_one(lines: &Vec<String>) -> u32 {
    let mut score = 0;

    for line in lines {
        let elf = RPS::from_letter(line.chars().next().unwrap()).expect("invalid entry");
        let mine = RPS::from_letter(line.chars().nth(2).unwrap()).expect("invalid entry");

        score += calculate_score(elf, mine)
    }

    score
}

pub fn part_two(lines: &Vec<String>) -> u32 {
    let mut score = 0;

    for line in lines {
        let elf = RPS::from_letter(line.chars().next().unwrap()).expect("invalid entry");
        let mine = match line.chars().nth(2).expect("invalid entry") {
            'X' => elf.win(),
            'Y' => elf,
            'Z' => elf.lose(),
            _ => panic!("invalid entry")
        };

        score += calculate_score(elf, mine);
    }

    score
}
