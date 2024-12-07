use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    let (a, b) = rationalise_input(&*f);
    let output = check_rules(a, b);
    let result: usize = output.iter().sum();

    dbg!(result);
}

fn check_rules(
    safety_instructions: Vec<(usize, usize)>,
    order_instructions: Vec<Vec<usize>>,
) -> Vec<usize> {
    // generate pairs from the order_instructions

    let mut middle_page: Vec<usize> = Vec::new();

    for order in order_instructions {
        let pairs: Vec<(usize, usize)> = order
            .windows(2)
            .map(|window| (window[0], window[1]))
            .collect();

        let order_instruction_pairs = pairs.clone().into_iter();
        let mut result_vec: Vec<bool> = Vec::new();

        for oip in order_instruction_pairs {
            for pair in &safety_instructions {
                if pair.0 == oip.0 && pair.1 == oip.1 {
                    // Good path
                    result_vec.push(true);
                }
            }
        }

        if result_vec.len() == order.len() - 1 {
            middle_page.push(order[order.len() / 2])
        }
    }
    middle_page
}

fn rationalise_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut safety_instructions: Vec<(usize, usize)> = vec![];
    let mut order_instructions: Vec<Vec<usize>> = vec![];

    if let Some(first_section) = sections.get(0) {
        for line in first_section.lines() {
            if let Some((k, v)) = line.split_once('|') {
                safety_instructions.push((k.parse().unwrap(), v.parse().unwrap()));
            }
        }
    }

    if let Some(second_section) = sections.get(1) {
        for line in second_section.lines() {
            let order: Vec<usize> = line.split(",").map(|val| val.parse().unwrap()).collect();
            order_instructions.push(order);
        }
    }

    (safety_instructions, order_instructions)
}
#[cfg(test)]
#[test]
fn test_order() {
    let payload = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    let (a, b) = rationalise_input(&payload);
    check_rules(a, b);
}
