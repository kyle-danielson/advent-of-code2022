use std::collections::HashSet;
use std::fs;

fn find_index_of_start_of_packet_marker(uniq_char: usize, buf: &str) -> u32 {
    let chars: Vec<char> = buf.chars().collect();
    for i in uniq_char..buf.len() {
        let mut uniq = HashSet::new();
        if chars[i - uniq_char..i].iter().all(|x| uniq.insert(x)) {
            return i as u32;
        }
    }
    0
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = file.trim().split("\n").collect();
    for &s in input.iter() {
        println!("Part 1: {}", find_index_of_start_of_packet_marker(4, s));
        println!("Part 2: {}", find_index_of_start_of_packet_marker(14, s));
    }
    // let _indexes = input.iter().map(|&s| {
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn part1() {
        let one = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let two = "nppdvjthqldpwncqszvftbrmjlhg";
        let three = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let four = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        let uniq_char = 4;
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, one), 5);
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, two), 6);
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, three), 10);
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, four), 11);
    }

    #[test]
    fn part2() {
        let one = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let two = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let three = "nppdvjthqldpwncqszvftbrmjlhg";
        let four = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let five = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        let uniq_char = 14;
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, one), 19);
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, two), 23);
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, three), 23);
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, four), 29);
        assert_eq!(find_index_of_start_of_packet_marker(uniq_char, five), 26);
    }
}
