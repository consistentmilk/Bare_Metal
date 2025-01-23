pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut lp: usize = 0;
        let mut rp: usize = height.len() - 1;
        let mut max_area: i32 = 0;

        while lp < rp {
            let area: i32 = ((rp - lp) as i32) * std::cmp::min(height[lp], height[rp]);
            max_area = std::cmp::max(max_area, area);

            match height[lp] < height[rp] {
                true => lp += 1,
                false => rp -= 1,
            }
        }

        max_area
    }

    pub fn max_area_self(height: Vec<i32>) -> i32 {
        let mut lp = 0;
        let mut rp = height.len() - 1;
        let mut max_area = 0;

        while lp < rp {
            max_area = std::cmp::max(
                max_area,
                std::cmp::min(height[lp], height[rp]) * (rp - lp) as i32,
            );

            if height[lp] < height[rp] {
                lp += 1;
            } else {
                rp -= 1;
            }
        }

        max_area
    }
}
