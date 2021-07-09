use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {

    let input = "../../files/day5_input.txt";
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut results: Vec<u32> = vec![];

    for lines in reader.lines() {
        let mut rows: Vec<u32>= (0..=127).into_iter().collect();
        let mut columns: Vec<u32>= (0..=7).into_iter().collect();
        let lines = lines.unwrap();
        let mut chars = lines.chars();
        let mut count = 0;

        while count != 10 {
            match &chars.next().unwrap() {
                'F' => rows = find_lower(rows),
                'B' => rows = find_upper(rows),
                'L' => columns = find_lower(columns),
                'R' => columns = find_upper(columns),
                _ => (),
            }
            count += 1;
        }

        let row = rows[0];
        let column = columns[0];
        let id = row * 8 + column;

        results.push(id);
    }

    let answer = results.iter().max().unwrap();

    println!("{}", answer)
}

fn find_upper(range: Vec<u32>) -> Vec<u32> {
    let mut new_range = range.clone();
    let length = new_range.len() / 2;
    let mut count = 0;

    while count != length {
        new_range.remove(0);
        count += 1;
    }
    new_range
}

fn find_lower(range: Vec<u32>) -> Vec<u32> {
    let mut new_range = range.clone();
    let length = new_range.len() / 2;
    let mut count = 0;

    while count != length {
        new_range.pop();
        count += 1;
    }
    new_range
}
