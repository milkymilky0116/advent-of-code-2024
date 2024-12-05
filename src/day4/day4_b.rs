use std::fs;

pub fn find_mas_puzzle(filename: String) -> usize {
    let file_content = fs::read_to_string(filename).expect("fail to read file");
    let xmax_vec = file_content
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;
    let target_vec = ['M', 'A', 'S'];
    for (col_index, col) in xmax_vec.iter().enumerate() {
        for (row_index, row) in col.iter().enumerate() {
            if *row == 'A'
                && col_index >= 1
                && col_index + 1 < xmax_vec.len()
                && row_index >= 1
                && row_index + 1 < col.len()
            {
                let mut mas_count = 0;
                if target_vec
                    == [
                        xmax_vec[col_index - 1][row_index - 1],
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index + 1][row_index + 1],
                    ]
                {
                    mas_count += 1;
                }

                if target_vec
                    == [
                        xmax_vec[col_index - 1][row_index + 1],
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index + 1][row_index - 1],
                    ]
                {
                    mas_count += 1;
                }

                if target_vec
                    == [
                        xmax_vec[col_index + 1][row_index + 1],
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index - 1][row_index - 1],
                    ]
                {
                    mas_count += 1;
                }

                if target_vec
                    == [
                        xmax_vec[col_index + 1][row_index - 1],
                        xmax_vec[col_index][row_index],
                        xmax_vec[col_index - 1][row_index + 1],
                    ]
                {
                    mas_count += 1;
                }
                if mas_count > 1 {
                    count += 1;
                }
            }
        }
    }
    count
}
