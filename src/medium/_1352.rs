pub struct ProductOfNumbers {
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    pub fn new() -> Self {
        Self { prefix: vec![1] }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix.clear();
            self.prefix.push(1);
        } else {
            let last: i32 = self.prefix.last().cloned().unwrap_or(1);
            self.prefix.push(last * num);
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let k: usize = k as usize;
        let n: usize = self.prefix.len();

        if k >= n {
            return 0;
        }

        // Product of all pushed elements before the k-th (starting at 1) element from the end
        let prod_k: i32 = self.prefix[n - (k + 1)];
        let total_product: i32 = self.prefix[n - 1];

        total_product / prod_k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1352_1() {
        let mut obj = ProductOfNumbers::new();

        obj.add(3);
        obj.add(0);
        obj.add(2);
        obj.add(5);
        obj.add(4);

        assert_eq!(obj.get_product(2), 20);
        assert_eq!(obj.get_product(3), 40);
        assert_eq!(obj.get_product(4), 0);

        obj.add(8);

        assert_eq!(obj.get_product(2), 32);
    }
}
