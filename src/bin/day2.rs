use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;


fn main() {
    let input_path = "../../files/day2_input.txt";
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);
    let re = Regex::new(r"^(?P<range1>\d+)-(?P<range2>\d+)\s(?P<key>\w):\s(?P<pass>\w+)").unwrap();
    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let start = caps.name("range1").unwrap().as_str().parse::<u32>().unwrap();
        let end = caps.name("range2").unwrap().as_str().parse::<u32>().unwrap();
        let key = caps.name("key").unwrap().as_str();
        let pass = caps.name("pass").unwrap().as_str();

        if pass.contains(key) {
            let c = pass.matches(key).count() as u32;
            if c >= start && c <= end {
                // println!("{}", line);
                count += 1;
            }
        }
    }
    println!("Number of results: {}", count);
}
