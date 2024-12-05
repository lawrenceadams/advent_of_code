use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input.txt").expect("opening input file");
    let reader = BufReader::new(f);

    let mut num_safe_systems = 0;

    for line in reader.lines() {
        let curr_line = line.unwrap();

        let parsed: Vec<i32> = curr_line
            .split_ascii_whitespace()
            .map(|val| val.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .expect("Unable to parse string to i32");

        if !is_monotonic(&parsed) {
            continue;
        }

        let mut iter_diff = parsed.iter().peekable();

        loop {
            match iter_diff.next() {
                Some(this_el) => match iter_diff.peek() {
                    Some(next_el) => match (this_el - *next_el).abs() {
                        1..=3 => continue,
                        _ => break,
                    },
                    None => num_safe_systems += 1,
                },
                _ => break,
            }
        }
    }
    dbg!(num_safe_systems);
}

fn is_monotonic(vec: &Vec<i32>) -> bool {
    let increasing = vec.windows(2).all(|w| w[0] <= w[1]);
    let decreasing = vec.windows(2).all(|w| w[0] >= w[1]);

    increasing || decreasing
}
