use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::collections::hash_set::Intersection;
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

fn get_value(x: char) -> u32 {
    if x.is_lowercase() {
        x as u32 - 'a' as u32 + 1
    } else {
        x as u32 - 'A' as u32 + 27
    }
}



fn main() {
    let mut score: u32 = 0;
    let mut p2score: u32 = 0;
    let mut group: Vec<HashSet<char>> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                

                let (bag1,bag2) = l.split_at(l.len()/2);

                let h1: HashSet<char> = HashSet::from_iter(bag1.chars());
                let h2: HashSet<char> = HashSet::from_iter(bag2.chars());

                let intersect = h1.intersection(&h2);
                let prio_score: u32 = intersect.map(|x| get_value(*x)).sum();         
                score += prio_score; 

                // part 2
                group.push(HashSet::from_iter(l.chars()));
                if group.len() == 3 {
                    if let Some(group_intersect) = group[0].intersection(&group[1]).find(|x| group[2].contains(x)) {
                        p2score += get_value(*group_intersect);
                    }
                    group.clear();
                }

            }
        }
        println!("total score: {}", score);
        println!("total badge score (p2): {}", p2score);

    }
}
