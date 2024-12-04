use std::fs;

pub fn parse_corrupted_memory(filename: String) -> i32 {
    let file_content = fs::read_to_string(filename)
        .expect("Fail to read file")
        .replace("\n", "")
        .replace("\r", "");

    let chars = file_content.chars().collect::<Vec<char>>();
    let mut result = 0;
    for (i, window) in chars.windows(4).enumerate() {
        if ['m', 'u', 'l', '('] == window {
            let temp_vec = &chars[i + 4..];
            if let Some(position) = temp_vec.iter().position(|&c| c == ')') {
                let substring = temp_vec[0..position]
                    .split(|&x| x == ',')
                    .collect::<Vec<&[char]>>();
                if let [a_part, b_part] = substring.as_slice() {
                    let a_str: String = a_part.iter().collect();
                    let b_str: String = b_part.iter().collect();
                    if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                        result += a * b;
                    }
                }
            }
        }
    }
    result
}
