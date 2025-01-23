use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: String = {
            let (first, last) = (
                word.chars().nth(0).unwrap(),
                word.chars().rev().nth(0).unwrap(),
            );
            let (mut l, mut r) = (0, 0);

            for ch in word.chars() {
                if ch == first {
                    l += 1;
                } else {
                    break;
                }
            }

            for ch in word.chars().rev() {
                if ch == last {
                    r += 1;
                } else {
                    break;
                }
            }

            if r < l {
                word.chars().rev().collect()
            } else {
                word
            }
        };

        let mut hset: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::path(&board, &word.as_bytes(), &mut hset, i, j, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn path(
        board: &Vec<Vec<char>>,
        word: &[u8],
        hset: &mut HashSet<(usize, usize)>,
        i: usize,
        j: usize,
        k: usize,
    ) -> bool {
        if k == word.len() {
            true
        } else if i >= board.len()
            || j >= board[0].len()
            || hset.contains(&(i, j))
            || board[i][j] as u8 != word[k]
        {
            false
        } else {
            hset.insert((i, j));

            let possible: bool = Self::path(board, word, hset, i + 1, j, k + 1)
                || Self::path(board, word, hset, i - 1, j, k + 1)
                || Self::path(board, word, hset, i, j + 1, k + 1)
                || Self::path(board, word, hset, i, j - 1, k + 1);
            hset.remove(&(i, j));

            possible
        }
    }
}
