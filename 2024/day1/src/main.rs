use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let p1: i32 = part_one(lines);
        println!("{}", p1);
    }

    if let Ok(lines) = read_lines(file_path) {
        let p2: i32 = part_two(lines);
        println!("{}", p2);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut buf_l: Vec<i32> = Vec::new();
    let mut buf_r: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    for line in lines.flatten() {
        let num1 = line.split_whitespace()
            .next()
            .expect("should not be None")
            .parse::<i32>()
            .unwrap();

        let num2 = line.split_whitespace()
            .next_back()
            .expect("should not be None")
            .parse::<i32>()
            .unwrap();

        buf_l.push(num1);
        buf_r.push(num2);
    }

    buf_l.sort();
    buf_r.sort();

    for (i, _el) in buf_l.iter().enumerate() {
        if buf_l[i] > buf_r[i] {
            total += buf_l[i] - buf_r[i];
        }

        if buf_l[i] < buf_r[i] {
            total += buf_r[i] - buf_l[i];
        }
    }

    total
}


fn part_two(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut buf_l: Vec<i32> = Vec::new();
    let mut buf_r: Vec<i32> = Vec::new();
    let mut total: i32 = 0;

    for line in lines.flatten() {
        let num1 = line.split_whitespace().next()
            .expect("should not be None")
            .parse::<i32>()
            .unwrap();

        let num2 = line.split_whitespace().next_back()
            .expect("should not be None")
            .parse::<i32>()
            .unwrap();

        buf_l.push(num1);
        buf_r.push(num2);
    }

    for i in buf_l {
        let mut occurences = 0;
        for j in &buf_r {
            if i == *j {
                occurences += 1;
            }
        }

        total += i * occurences;
    }

    total
}

