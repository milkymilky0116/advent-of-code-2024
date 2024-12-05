use std::fs;

pub fn find_xmas(filename: String) -> u32 {
    let file_content = fs::read_to_string(filename).expect("fail to read file");
    let xmax_vec = file_content
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;
    let target_vec = ['X', 'M', 'A', 'S'];
    for (col_index, col) in xmax_vec.iter().enumerate() {
        for (row_index, row) in col.iter().enumerate() {
            if *row == 'X' {
                //forward
                if row_index + 4 <= col.len() {
                    if target_vec == &col[row_index..row_index + 4] {
                        count += 1;
                    }
                }
                //backward
                if row_index >= 3 {
                    let mut reverse_slice = col[row_index - 3..row_index + 1].to_vec();
                    reverse_slice.reverse();
                    if target_vec == reverse_slice.as_slice() {
                        count += 1;
                    }
                }

                //down
                if col_index + 4 <= xmax_vec.len() {
                    let down_slice = [
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index + 1][row_index],
                        xmax_vec[col_index + 2][row_index],
                        xmax_vec[col_index + 3][row_index],
                    ];
                    if target_vec == down_slice {
                        count += 1;
                    }
                }

                //up
                if col_index >= 3 {
                    let mut up_slice = [
                        xmax_vec[col_index - 3][row_index],
                        xmax_vec[col_index - 2][row_index],
                        xmax_vec[col_index - 1][row_index],
                        xmax_vec[col_index][row_index],
                    ];
                    up_slice.reverse();
                    if target_vec == up_slice {
                        count += 1;
                    }
                }

                //down-right
                if col_index + 4 <= xmax_vec.len() && row_index + 4 <= col.len() {
                    let down_right_slice = [
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index + 1][row_index + 1],
                        xmax_vec[col_index + 2][row_index + 2],
                        xmax_vec[col_index + 3][row_index + 3],
                    ];
                    if target_vec == down_right_slice {
                        count += 1;
                    }
                }

                //down-left
                if col_index + 3 < xmax_vec.len() && row_index >= 3 {
                    let down_left_slice = [
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index + 1][row_index - 1],
                        xmax_vec[col_index + 2][row_index - 2],
                        xmax_vec[col_index + 3][row_index - 3],
                    ];
                    if target_vec == down_left_slice {
                        count += 1;
                    }
                }

                //up-right
                if col_index >= 3 && row_index + 3 < col.len() {
                    let up_right_slice = [
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index - 1][row_index + 1],
                        xmax_vec[col_index - 2][row_index + 2],
                        xmax_vec[col_index - 3][row_index + 3],
                    ];
                    if target_vec == up_right_slice {
                        count += 1;
                    }
                }

                //up-left
                if col_index >= 3 && row_index >= 3 {
                    let up_left_slice = [
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index - 1][row_index - 1],
                        xmax_vec[col_index - 2][row_index - 2],
                        xmax_vec[col_index - 3][row_index - 3],
                    ];
                    if target_vec == up_left_slice {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
