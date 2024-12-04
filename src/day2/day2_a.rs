use std::fs;

pub fn day2_b(filename: String) -> i32 {
    let file_content = fs::read_to_string(filename).expect("Fail to read file");
    let lines: Vec<Vec<i32>> = file_content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().expect("Fail to parse str to i32"))
                .collect::<Vec<i32>>()
        })
        .collect();
    lines
        .iter()
        .filter(|line| {
            let mut has_increase = false;
            let mut has_decrease = false;
            for window in line.windows(2) {
                let (left_val, right_val) = (window[0], window[1]);
                let diff = (left_val - right_val).abs();
                if diff > 3 || diff == 0 || (has_increase && has_decrease) {
                    return false;
                }
                if left_val < right_val {
                    has_increase = true;
                } else {
                    has_decrease = true;
                }
            }
            true
        })
        .count() as i32
}
