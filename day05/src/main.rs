use std::fs;

fn parse_crates(crates: Vec<&str>) -> Vec<String> {
    let mut ret: Vec<String> = vec![String::from("")];
    for line in crates.iter() {
        for (i, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if ret.len() <= i {
                ret.push(String::new())
            }
            if chunk.starts_with(&['[']) {
                ret[i].push(chunk[1]);
            }
        }
    }
    let ret: Vec<String> = ret
        .iter()
        .map(|x| x.chars().rev().collect::<String>())
        .collect();
    ret
}

fn do_operations(mut crates: Vec<String>, instructions: Vec<&str>) -> Vec<String> {
    let inst: Vec<(usize, usize, usize)> = instructions
        .iter()
        .map(|l| {
            let l: Vec<&str> = l.split(" ").collect();
            (
                l[1].parse::<usize>().unwrap(),
                l[3].parse::<usize>().unwrap() - 1,
                l[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    
    for i in inst {
        let mut str: String = String::new();
        for _ in 0..i.0 {
            
            str.push(crates[i.1].pop().unwrap())
            // Part 1
            // let ch = crates[i.1].pop().unwrap();
            // crates[i.2].push(ch);
        }
        crates[i.2].push_str(str.chars().rev().collect::<String>().as_str());
    }
    crates
}

fn get_p1_answ(crates: Vec<String>) -> String{
    let mut s =  String::new();
    for (_e, c) in crates.iter().enumerate() {
        s.push(c.chars().last().unwrap_or(' '));
        // println!("Crate: {} - Last Item {}", e, c.chars().last().unwrap());
    }
    println!("{}", s);
    s
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let (mut crates, instructions): (Vec<_>, Vec<_>) =
        file.lines().partition(|&l| !l.starts_with("move"));
    crates.truncate(&crates.len() - 2);

    let mut c = parse_crates(crates);
    c = do_operations(c, instructions);
    get_p1_answ(c);

    // print!("{:?}", lines.find(""));
    // let (header, instructions) = lines.split("");
    // print!("{} {}", header.len(), instructions.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1() {
        use crate::*;

        let file: String = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        let (mut crates, instructions): (Vec<_>, Vec<_>) =
            file.lines().partition(|&l| !l.starts_with("move"));
        crates.truncate(&crates.len() - 2);

        let mut c = parse_crates(crates);
        c = do_operations(c, instructions);
        let s = get_p1_answ(c);

        assert_eq!(s, String::from("CMZ"));
    }

    #[test]
    fn p2() {
        use crate::*;

        let file: String = String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );

        let (mut crates, instructions): (Vec<_>, Vec<_>) =
            file.lines().partition(|&l| !l.starts_with("move"));
        crates.truncate(&crates.len() - 2);

        let mut c = parse_crates(crates);
        c = do_operations(c, instructions);
        let s = get_p1_answ(c);

        assert_eq!(s, String::from("MCD"));
    }
}
