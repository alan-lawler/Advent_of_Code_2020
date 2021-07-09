use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let input_path = "../../files/day1_input.txt";
    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);
    // let mut list = Vec::new();

    let list = reader
        .lines()
        .map(|lines| lines.unwrap().trim().parse().unwrap()).collect();

    // same as above...
    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     let n = line.trim().parse::<u32>().unwrap();
    //     list.push(n);
    // }

    let part1 = read(&list);
    let part2 = read_part_two(&list);
    println!("Part 1: {}", part1.unwrap());
    println!("Part 2: {}", part2.unwrap());
}

fn read(list: &Vec<u32>) -> Option<u32> {
    let list = list.clone();
    for number1 in &list {
        for number2 in &list {
            if number1 + number2 == 2020 {
                println!("Number 1: {}", number1);
                println!("Number 2: {}", number2);
                return Some(number1 * number2)
            }
        }
    }
    None
}

fn read_part_two(list: &Vec<u32>) -> Option<u32> {
    let list = list.clone();
    for number1 in &list {
        for number2 in &list {
            for number3 in &list {
                if number1 + number2 + number3 == 2020 {
                    println!("Number 1: {}", number1);
                    println!("Number 2: {}", number2);
                    println!("Number 2: {}", number3);
                    return Some(number1 * number2 * number3)
                }
            }
        }
    }
    None
}