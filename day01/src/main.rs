use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut cal_count: u32 = 0;
        let mut elf_vec: Vec<u32> = vec![];
        for line in lines {
            if let Ok(l) = line {
                if l.is_empty() {
                    elf_vec.push(cal_count);
                    cal_count = 0;
                    continue;
                }

                cal_count += l.parse::<u32>().unwrap();
            }
        }
        // part 1
        println!(
            "Most calories carried per elf: {}",
            elf_vec.iter().max().unwrap()
        );
        // part 2
        elf_vec.sort_by(|a, b| b.cmp(a));
        let top_three_cal_sum: u32 = elf_vec.iter().take(3).sum::<u32>();
        println!("Calories carried by top 3 elves: {}", top_three_cal_sum);
    }
}
