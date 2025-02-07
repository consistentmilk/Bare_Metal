use std::collections::{hash_map::Entry, HashMap, HashSet};

pub struct Solution;

impl Solution {
    ///
    /// Algorithm: Time Complexity: O(n) | Space Complexity: O(n)
    /// Input: _limit: i32, queries: Vec<Vec<i32>>
    /// 1. Initialize a Vec 'res' with capacity equalto the length of 'queries'
    /// 2. Initialize a HashMap 'ball_colors' with (key, value) = (i32, i32) and dynamic memory allocation
    /// 3. Initialize a HashMap 'color_counts' with (key, value) = (i32, i32) and dynamic memory allocation
    /// 4. Iterate over queries to get individual 'query: Vec<i32> | [Index, Color]' at each step
    ///     - Insert key query[0] (Index) into 'ball_colors'
    ///         - If key was not previously in 'ball_colors', retrieve VacantEntry as 'ball_color'
    ///             - Insert corresponding value: query[1] (Color) to complete the key value pair into 'ball_color'
    ///             - Insert entry query[1] (Color) into 'color_counts' and get a mutable borrow of its corresponding value as 'color_count',
    ///               if query[1] was not previously a key in 'color_counts', insert 0 as its corresponding value
    ///             - Increment color_count by 1
    ///         - If key was previously in 'ball_colors', retrieve OccupiedEntry as mut 'ball_color'
    ///             - Replace "ball_color" with query[1] and get retrieve its previous value as 'old_color'
    ///             - If 'old_color' and current Color/query[1] are not equal
    ///                  - Insert entry query[1] (Color) into 'color_counts' and get a mutable borrow of its corresponding value as 'color_count',
    ///                    if query[1] was not previously a key in 'color_counts', insert 0 as its corresponding value
    ///                  - Increment color_count by 1
    ///
    ///
    /// [[1,4], [2,5], [1,3], [3,4]]
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(queries.len());

        // (Key, Value) = (Index, Color)
        let mut ball_colors: HashMap<i32, i32> = HashMap::new();

        // (Key, Value) = (Color, Color Freq)
        let mut color_counts: HashMap<i32, i32> = HashMap::new();

        for query in queries {
            // query = [Index, Color]
            match ball_colors.entry(query[0]) {
                // Ball at Index not assigned color yet
                Entry::Vacant(ball_color) => {
                    ball_color.insert(query[1]);

                    let color_freq = color_counts.entry(query[1]).or_insert(0);
                    *color_freq += 1;
                }

                // Ball at Index already colored
                Entry::Occupied(mut ball_color) => {
                    // The current Color we are assigning to Index
                    let curr_color = query[1];
                    // The Color that was previously assigned to this Index
                    let old_color = std::mem::replace(ball_color.get_mut(), curr_color);

                    // If current color and old color are same, we don't need to change color freq count
                    if old_color != curr_color {
                        let curr_color_freq = color_counts.entry(curr_color).or_insert(0);
                        *curr_color_freq += 1;

                        // Decrement color freq of old Color by 1, if freq is after decrement, remove the corresponding key, decreasing
                        // the length of the color counts HashMap by 1
                        match color_counts.entry(old_color) {
                            Entry::Occupied(mut old_color_freq) => {
                                *old_color_freq.get_mut() -= 1;

                                if *old_color_freq.get() == 0 {
                                    color_counts.remove(&old_color);
                                }
                            }

                            Entry::Vacant(_) => {}
                        }
                    }
                }
            }

            res.push(color_counts.len() as i32);
        }

        res
    }
}
pub struct SolutionAltOne;

impl SolutionAltOne {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_colors: HashMap<i32, i32> = HashMap::new(); // Stores color for each ball
        let mut color_balls: HashMap<i32, HashSet<i32>> = HashMap::new(); // Maps color to balls with that color

        let mut result: Vec<i32> = Vec::new();

        for query in queries {
            let x: i32 = query[0]; // Ball label
            let y: i32 = query[1]; // Color

            // If the ball already has a color, remove it from the old color's set
            if let Some(&old_color) = ball_colors.get(&x) {
                if let Some(balls) = color_balls.get_mut(&old_color) {
                    balls.remove(&x);
                    if balls.is_empty() {
                        color_balls.remove(&old_color);
                    }
                }
            }

            // Assign the new color to the ball
            ball_colors.insert(x, y);
            color_balls.entry(y).or_insert_with(HashSet::new).insert(x);

            // Count distinct colors by checking the non-empty color sets
            result.push(color_balls.len() as i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3160_1() {
        let _limit = 4;
        let queries: Vec<Vec<i32>> = [[1, 4], [2, 5], [1, 3], [3, 4]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect();
        let expected: Vec<i32> = vec![1, 2, 2, 3];

        assert_eq!(expected, Solution::query_results(_limit, queries));
    }

    #[test]
    fn test_3160_2() {
        let _limit = 4;
        let queries: Vec<Vec<i32>> = [[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]]
            .into_iter()
            .map(|x: [i32; 2]| x.to_vec())
            .collect();
        let expected: Vec<i32> = vec![1, 2, 2, 3, 4];

        assert_eq!(expected, Solution::query_results(_limit, queries));
    }
}
