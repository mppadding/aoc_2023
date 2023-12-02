use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_file_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn part_1(lines: &Vec<String>) {
    let result = lines.iter().fold(0, |acc, l| {
        let first = l
            .chars()
            .find(|&c| c.to_digit(10).is_some())
            .map(|v| v.to_digit(10).unwrap() * 10)
            .expect("No first digit found in line `{l}`");

        let last = l
            .chars()
            .rfind(|&c| c.to_digit(10).is_some())
            .map(|v| v.to_digit(10).unwrap())
            .expect("No last digit found in line `{l}`");

        acc + (first + last)
    });

    println!("part_1 = {result}");
}

fn main() {
    let lines = read_file_lines("input/part_1");

    part_1(&lines);
}
