use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input.txt").expect("opening input file");
    let reader = BufReader::new(f);

    let mut vec_1: Vec<i32> = Vec::new();
    let mut vec_2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let x = line.unwrap();
        let mut iter = x.split_ascii_whitespace();

        let col_left = iter.next().unwrap().parse::<i32>().unwrap();
        let col_right = iter.next().unwrap().parse::<i32>().unwrap();

        vec_1.push(col_left);
        vec_2.push(col_right);
    }

    vec_1.sort();
    vec_2.sort();

    let zipped = vec_1.iter().zip(vec_2.iter());

    let mut total_diff = 0;

    for a in zipped {
        total_diff += (a.0 - a.1).abs();
    }

    println!("Result: {total_diff}");
}
