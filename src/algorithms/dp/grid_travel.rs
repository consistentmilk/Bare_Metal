use std::collections::HashMap;

pub struct GridTraveler;

impl GridTraveler {
    pub fn grid_travel(m: usize, n: usize) -> usize {
        let mut mem_tbl: HashMap<String, usize> = HashMap::new();

        Self::recursion_helper(m, n, &mut mem_tbl)
    }

    fn recursion_helper(m: usize, n: usize, mem_tbl: &mut HashMap<String, usize>) -> usize {
        let key: String = format!("{},{}", m, n);

        if let Some(&memoized) = mem_tbl.get(&key) {
            return memoized;
        }

        if m == 1 && n == 1 {
            return 1;
        }

        if m == 0 || n == 0 {
            return 0;
        }

        let res: usize =
            Self::recursion_helper(m - 1, n, mem_tbl) + Self::recursion_helper(m, n - 1, mem_tbl);

        mem_tbl.insert(key, res);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpgt_1() {
        assert_eq!(GridTraveler::grid_travel(1, 1), 1);

        assert_eq!(GridTraveler::grid_travel(2, 3), 3);

        assert_eq!(GridTraveler::grid_travel(3, 2), 3);

        assert_eq!(GridTraveler::grid_travel(3, 3), 6);

        assert_eq!(GridTraveler::grid_travel(18, 18), 2333606220);
    }
}
