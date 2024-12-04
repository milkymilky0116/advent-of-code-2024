use std::fs;

pub fn day1_a(file_name: String) -> i32 {
    let mut left_nums = vec![];
    let mut right_nums = vec![];
    fs::read_to_string(file_name)
        .expect("Fail to read file")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .for_each(|arr| {
            let left_val = arr
                .get(0)
                .expect("Fail to get left value")
                .parse::<i32>()
                .expect("Fail to cast to i64");

            let right_val = arr
                .get(1)
                .expect("Fail to get left value")
                .parse::<i32>()
                .expect("Fail to cast to i64");
            left_nums.push(left_val);
            right_nums.push(right_val);
            left_nums.sort();
            right_nums.sort();
        });
    let mut result = 0;
    for i in 0..left_nums.len() {
        let left_small_val = left_nums.get(i).expect("Fail to get left array value");

        let right_small_val = right_nums.get(i).expect("Fail to get left array value");
        result += (left_small_val - right_small_val).abs();
    }
    result
}
