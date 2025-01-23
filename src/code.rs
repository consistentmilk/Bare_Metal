pub fn convert(s: String, num_rows: i32) -> String {
    let num_rows: usize = num_rows as usize;

    if num_rows == 1 || num_rows >= s.len() {
        return s;
    }

    let mut current_row: usize = 0;
    let mut dp: Vec<String> = vec!["".into(); num_rows];
    let mut direction: i32 = -1;

    for ch in s.chars() {
        dp[current_row].push(ch);

        if current_row == 0 || current_row == num_rows - 1 {
            direction *= -1;
        }

        current_row = (current_row as i32 + direction) as usize;
    }

    dp.iter()
        .flat_map(|s: &String| s.as_str().chars())
        .collect()
}
