use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows: usize = board.len();

        if rows == 0 {
            return false;
        }

        let cols: usize = board[0].len();
        if word.len() > rows * cols {
            return false;
        }

        let mut board_count: HashMap<char, usize> = HashMap::new();

        for row in &board {
            for &ch in row {
                *board_count.entry(ch).or_insert(0) += 1;
            }
        }

        let mut word_chars: Vec<char> = word.chars().collect();
        let mut word_count: HashMap<char, usize> = HashMap::new();

        for &ch in &word_chars {
            *word_count.entry(ch).or_insert(0) += 1;
        }

        for (ch, count_word) in &word_count {
            if board_count.get(ch).cloned().unwrap_or(0) < *count_word {
                return false;
            }
        }

        if board_count.get(&word_chars[0]).unwrap_or(&0)
            > board_count
                .get(&word_chars[word_chars.len() - 1])
                .unwrap_or(&0)
        {
            word_chars.reverse();
        }

        let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                if Self::dfs(&board, &word_chars, &mut visited, i as i32, j as i32, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
        r: i32,
        c: i32,
        i: usize,
    ) -> bool {
        let rows: i32 = board.len() as i32;
        let cols: i32 = board[0].len() as i32;

        if i == word.len() {
            return true;
        }

        if r < 0 || c < 0 || r >= rows || c >= cols {
            return false;
        }

        let r_usize: usize = r as usize;
        let c_usize: usize = c as usize;

        if visited[r_usize][c_usize] || board[r_usize][c_usize] != word[i] {
            return false;
        }

        visited[r_usize][c_usize] = true;

        let found = Self::dfs(board, word, visited, r + 1, c, i + 1)
            || Self::dfs(board, word, visited, r - 1, c, i + 1)
            || Self::dfs(board, word, visited, r, c + 1, i + 1)
            || Self::dfs(board, word, visited, r, c - 1, i + 1);

        visited[r_usize][c_usize] = false;

        found
    }

    pub fn exist_iterative_optimized(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let rows: usize = board.len();

        if rows == 0 {
            return false;
        }

        let cols: usize = board[0].len();
        let word_len: usize = word.len();

        // Ultimate pruning
        if word_len == 0 {
            return true;
        }

        if word_len > rows * cols {
            return false;
        }

        // Frequency check with early exit
        let mut board_freq: [i32; 128] = [0; 128];
        let mut word_freq: [i32; 128] = [0; 128];
        board
            .iter()
            .flatten()
            .for_each(|&c| board_freq[c as usize] += 1);
        word.iter().for_each(|&c| word_freq[c as usize] += 1);

        if word
            .iter()
            .any(|&c| word_freq[c as usize] > board_freq[c as usize])
        {
            return false;
        }

        // Choose starting direction based on word structure
        let (start_char, end_char) = (word[0], word[word_len - 1]);

        let reverse_search: bool = word_len > 1 && {
            let start_count: usize = word.iter().take_while(|&&c| c == start_char).count();
            let end_count: usize = word.iter().rev().take_while(|&&c| c == end_char).count();
            end_count < start_count
        };

        let search_word: Vec<char> = if reverse_search {
            word.iter().rev().cloned().collect()
        } else {
            word
        };

        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == search_word[0] {
                    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
                    let mut stack: Vec<(usize, usize, usize, bool)> = Vec::with_capacity(word_len);
                    visited[i][j] = true;
                    stack.push((i, j, 0, false));

                    while let Some((x, y, k, backtrack)) = stack.pop() {
                        if backtrack {
                            visited[x][y] = false;
                            continue;
                        }

                        if k == word_len - 1 {
                            return true;
                        }

                        visited[x][y] = true;
                        stack.push((x, y, k, true)); // Backtrack marker

                        for &(dx, dy) in &directions {
                            let nx: i32 = x as i32 + dx;
                            let ny: i32 = y as i32 + dy;

                            if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                                let nx: usize = nx as usize;
                                let ny: usize = ny as usize;

                                if !visited[nx][ny] && board[nx][ny] == search_word[k + 1] {
                                    stack.push((nx, ny, k + 1, false));
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_079_1() {
        let board: Vec<Vec<char>> = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .iter()
        .map(|v: &[char; 4]| v.to_vec())
        .collect();
        let word: String = "ABCCED".into();

        assert!(Solution::exist(board.clone(), word.clone()));
        assert!(Solution::exist_iterative_optimized(board, word));
    }

    #[test]
    fn test_079_2() {
        let board: Vec<Vec<char>> = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .iter()
        .map(|v: &[char; 4]| v.to_vec())
        .collect();
        let word: String = "SEE".into();

        assert!(Solution::exist(board.clone(), word.clone()));
        assert!(Solution::exist_iterative_optimized(board, word));
    }

    #[test]
    fn test_079_3() {
        let board: Vec<Vec<char>> = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ]
        .iter()
        .map(|v: &[char; 4]| v.to_vec())
        .collect();
        let word: String = "ABCB".into();

        assert!(!Solution::exist(board.clone(), word.clone()));
        assert!(!Solution::exist_iterative_optimized(board, word));
    }
}
