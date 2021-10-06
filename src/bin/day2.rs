use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;


fn main() {
    let input_path = "../../files/day2_input.txt";
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(&file);
    let (p1, p2) = part_one(reader);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part_one(reader: BufReader<&File>) -> (i32, i32) {
    let re = Regex::new(r"^(?P<start>\d+)-(?P<end>\d+)\s(?P<key>\w):\s(?P<pass>\w+)").unwrap();
    let mut count = 0;
    let mut count_p2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let start = caps.name("start").unwrap().as_str().parse::<usize>().unwrap();
        let end = caps.name("end").unwrap().as_str().parse::<usize>().unwrap();
        let key = caps.name("key").unwrap().as_str();
        let pass = caps.name("pass").unwrap().as_str();

        if pass.contains(key) {
            let c = pass.matches(key).count() as usize;
            if c >= start && c <= end {
                count += 1;
            }

            let password: Vec<char> = pass.chars().collect();
            if password[start - 1].to_owned().to_string() == key && password[end - 1].to_owned().to_string() != key{
                count_p2 += 1;
            }
            if password[start - 1].to_owned().to_string() != key && password[end - 1].to_owned().to_string() == key {
                count_p2 += 1;
            }
        }
    }
    (count, count_p2)
}