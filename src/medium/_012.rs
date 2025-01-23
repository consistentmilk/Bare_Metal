pub struct Solution {}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let units: [(&str, i32); 13] = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut result: String = String::with_capacity(15);
        for (unit, val) in units.into_iter() {
            while num >= val {
                result.push_str(unit);
                num -= val;
            }
        }

        result
    }
}
