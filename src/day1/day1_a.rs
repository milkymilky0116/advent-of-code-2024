use std::fs;

pub fn day1_a(file_name: String) -> i32 {
    let mut left_nums = vec![];
    let mut right_nums = vec![];
    let file_content = fs::read_to_string(file_name).expect("Fail to read file");
    file_content.lines().for_each(|line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|value| value.parse::<i32>().expect("Fail to parse to i32"))
            .collect();
        if let [left, right] = nums.as_slice() {
            left_nums.push(*left);
            right_nums.push(*right);
        }
    });
    left_nums.sort_unstable();
    right_nums.sort_unstable();
    left_nums
        .iter()
        .zip(right_nums.iter())
        .map(|(left, right)| (left - right).abs())
        .sum()
}
