pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut perms: Vec<Vec<String>> = Vec::new();

        Self::get_perms(&mut (0..n).into_iter().collect(), 0, n as usize, &mut perms);

        perms
    }

    pub fn get_perms(
        nums: &mut Vec<i32>,
        nums_used: usize,
        nums_remaining: usize,
        perms: &mut Vec<Vec<String>>,
    ) {
        if nums_used < 2 || Self::is_valid_perm(nums, nums_used) {
            if nums_remaining == 1 && Self::is_valid_perm(nums, nums_used + 1) {
                perms.push(Self::perm_to_string(nums));
            } else {
                for i in nums_used..nums.len() {
                    nums.swap(nums_used, i);
                    Self::get_perms(nums, nums_used + 1, nums_remaining - 1, perms);
                    nums.swap(i, nums_used);
                }
            }
        }
    }

    pub fn perm_to_string(perm: &Vec<i32>) -> Vec<String> {
        let mut board_string: Vec<String> = Vec::with_capacity(perm.len());

        for &num in perm {
            let mut row: Vec<u8> = vec!['.' as u8; perm.len()];
            row[num as usize] = 'Q' as u8;
            board_string.push(String::from_utf8(row).unwrap());
        }

        board_string
    }

    pub fn is_valid_perm(perm: &Vec<i32>, length: usize) -> bool {
        let row: usize = length - 1;
        let col: i32 = perm[row];

        for i in 0..row {
            if (row - i) == (col - perm[i]).abs() as usize {
                return false;
            }
        }

        true
    }
}
