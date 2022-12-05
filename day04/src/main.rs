use std::fs;

fn range_fully_overlapped(elf1: &Vec<u32>, elf2: &Vec<u32>) -> bool {
    match (elf1, elf2) {
        (a, b) if a[0] >= b[0] && a[1] <= b[1] => true,
        (a, b) if b[0] >= a[0] && b[1] <= a[1] => true,
        (_, _) => false,
    }
}

fn partial_overlap(elf1: &Vec<u32>, elf2: &Vec<u32>) -> bool {
    match (elf1, elf2) {
        (a, b) if a[0] <= b[1] && a[1] >= b[0] => true,
        (_, _) => false,
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = file.trim().split("\n").collect();
    let mut count: u32 = 0;
    let mut count2: u32 = 0;


    for squad in input {
        let elves: Vec<Vec<u32>> = squad
            .split(",")
            .map(|x| x.split("-").map(|y| y.parse::<u32>().unwrap()).collect())
            .collect();

        // part 1
        if range_fully_overlapped(&elves[0], &elves[1]) {
            count += 1;
        }

        // part 2
        if partial_overlap(&elves[0], &elves[1]) {
            count2 += 1;
        }
    }

    println!("total overlapping sets: {}", count);
    println!("partial overlapping sets: {}", count2);
}
