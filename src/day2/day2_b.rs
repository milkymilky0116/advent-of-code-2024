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
        .filter(|line| check_safety_with_tolerance(&line))
        .count() as i32
}

fn check_safety(line: &Vec<i32>) -> bool {
    let is_asending = line.windows(2).all(|w| w[0] <= w[1]);
    let is_desending = line.windows(2).all(|w| w[0] >= w[1]);
    if !is_asending && !is_desending {
        return false;
    }
    let check_in_range = line
        .windows(2)
        .map(|w| (w[0] - w[1]).abs())
        .all(|x| x <= 3 && x >= 1);
    check_in_range
}

fn check_safety_with_tolerance(line: &Vec<i32>) -> bool {
    if check_safety(line) {
        return true;
    }
    for i in 0..line.len() {
        let mut filtered_vec = line.clone();
        filtered_vec.remove(i);
        if check_safety(&filtered_vec) {
            return true;
        }
    }
    false
}
