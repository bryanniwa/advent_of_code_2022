use std::io;

pub fn single_elf(lines: &Vec<String>) -> io::Result<u32> {
    let mut total = 0;
    let mut largest = 0;
    for line in lines {
        if line.len() > 1 {
            total += line.parse::<u32>().unwrap();
        } else {
            if total > largest {
                largest = total;
            }
            total = 0;
        }
    }

    Ok(largest)
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn top_three(lines: &Vec<String>) -> io::Result<u32> {
    let mut total = 0;
    let mut top_elves: BinaryHeap<Reverse<_>> = vec![0, 0, 0].into_iter().map(Reverse).collect();

    for line in lines {
        if line.len() > 1 {
            total += line.parse::<u32>().unwrap();
        } else {
            if let Some(Reverse(v)) = top_elves.peek() {
                if total > *v {
                    top_elves.pop();
                    top_elves.push(Reverse(total))
                }
            } else {
                top_elves.push(Reverse(total))
            }
            total = 0;
        }
    }

    let mut grand_total = 0;
    while let Some(Reverse(v)) = top_elves.pop() {
        grand_total += v;
    }

    Ok(grand_total)
}
