use std::{collections::HashMap, fs};

pub fn day1_b(filename: String) -> i32 {
    let file_content = fs::read_to_string(filename).expect("Fail to find file");

    let mut left_val = vec![];
    let mut right_val = HashMap::new();
    file_content.lines().for_each(|line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|value| value.parse::<i32>().expect("Fail to parse str to i32"))
            .collect();
        if let [left, right] = nums.as_slice() {
            left_val.push(*left);
            *right_val.entry(*right).or_insert(0) += 1;
        }
    });
    left_val
        .iter()
        .map(|value| value * right_val.get(value).unwrap_or(&0))
        .sum()
}
