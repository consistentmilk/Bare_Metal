use std::collections::HashMap;

pub struct Fibonacci;

impl Fibonacci {
    pub fn nth_fib(n: usize) -> usize {
        let mut mem_tbl: HashMap<usize, usize> = HashMap::new();

        Self::recursion_helper(n, &mut mem_tbl)
    }

    pub fn recursion_helper(n: usize, mem_tbl: &mut HashMap<usize, usize>) -> usize {
        if let Some(&memoized) = mem_tbl.get(&n) {
            return memoized;
        }

        if n <= 2 {
            return 1;
        }

        let res: usize =
            Self::recursion_helper(n - 1, mem_tbl) + Self::recursion_helper(n - 2, mem_tbl);
        mem_tbl.insert(n, res);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpfib() {
        assert_eq!(Fibonacci::nth_fib(6), 8);

        assert_eq!(Fibonacci::nth_fib(7), 13);

        assert_eq!(Fibonacci::nth_fib(8), 21);

        assert_eq!(Fibonacci::nth_fib(50), 12586269025);
    }
}
