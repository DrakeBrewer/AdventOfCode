use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];



    if let Ok(lines) = read_lines(file_path) {
        let mut total = 0;
        for i in lines.flatten() {
            let levels: Vec<i32> = i.split(" ").map(|x|->i32{x.parse().unwrap()}).collect();

            if part_one(&levels) {
                total += 1;
            }

        }
        println!("{}", total);
    }

    if let Ok(lines) = read_lines(file_path) {
        let mut total = 0;
        for i in lines.flatten() {
            let levels: Vec<i32> = i.split(" ").map(|x|->i32{x.parse().unwrap()}).collect();

            if part_two(&levels) {
                total += 1;
            }

        }
        println!("{}", total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(levels: &Vec<i32>) -> bool {
    let mut increasing = true;
    for (j, _lvl) in levels.iter().enumerate() {
        if j > 0 {
            let diff = levels[j] - levels[j-1];
            if diff.abs() > 3 || diff == 0 {
                return false
            }

            if diff < 0 && j == 1 {
                increasing = false;
            }

            if j > 1 {
                if increasing {
                    if diff < 0 {
                        return false;
                    }
                }

                if !increasing {
                    if diff > 0 {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn part_two(levels: &Vec<i32>) -> bool {
    if part_one(&levels) {
        return true
    }

    for i in 0..levels.len() {
        let mut tmp_lvl = levels.clone();
        tmp_lvl.remove(i);

        if part_one(&tmp_lvl) {
            return true
        }
    }

    false
}
