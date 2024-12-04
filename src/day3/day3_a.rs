use std::fs;

pub fn parse_corrupted_memory(filename: String) -> i32 {
    let file_content = fs::read_to_string(filename)
        .expect("Fail to read file")
        .replace("\n", "")
        .replace("\r", "");
    file_content
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            let start = &file_content[i + 4..];
            let end = start.find(")")?;
            let (a, b) = &start[..end].split_once(",")?;
            let num1 = a.parse::<i32>().ok()?;
            let num2 = b.parse::<i32>().ok()?;
            Some(num1 * num2)
        })
        .sum()
}
