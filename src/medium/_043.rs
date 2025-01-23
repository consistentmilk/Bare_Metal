pub struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1.as_str() == "0" || num2.as_str() == "0" {
            return "0".into();
        }

        let bytes1: &[u8] = num1.as_bytes();
        let bytes2: &[u8] = num2.as_bytes();

        let mut products: Vec<u32> = vec![0; bytes1.len() + bytes2.len()];

        for i in 0..bytes1.len() {
            for j in 0..bytes2.len() {
                products[i + j + 1] += ((bytes1[i] - 48) * (bytes2[j] - 48)) as u32;
            }
        }

        let mut carry: u32 = 0;
        for i in (0..products.len()).rev() {
            let tmp: u32 = (products[i] + carry) % 10;
            carry = (products[i] + carry) / 10;
            products[i] = tmp;
        }

        String::from_iter(
            products
                .into_iter()
                .map(|val: u32| std::char::from_digit(val, 10).expect("Failed to parse.")),
        )
        .trim_start_matches('0')
        .into()
    }
}
