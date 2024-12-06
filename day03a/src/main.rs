use regex;
use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();

    dbg!(parse_mul_queries(&f));
}

fn parse_mul_queries(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut result: i32 = 0;
    for cap in re.captures_iter(input) {
        if let Some(matched) = cap.get(0) {
            let parsed = matched.as_str();
            let finalresult = parsed.replace("mul(", "").replace(")", "");
            let nums: Vec<i32> = finalresult
                .split(',')
                .map(|val| val.parse().unwrap())
                .collect();

            result += nums[0] * nums[1];
        }
    }

    result
}

#[cfg(test)]
#[test]
fn test_parse_mul_queries() {
    let payload = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    assert_eq!(parse_mul_queries(payload), 161);
}
