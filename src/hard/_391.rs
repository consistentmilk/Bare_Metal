//! # Perfect Rectangle
//!
//! ## Intuition
//! 1. A set of small rectangles exactly forms a perfect rectangle if:
//!    - The total area of the small rectangles equals the area of the bounding rectangle.
//!    - The only corner points that appear exactly once are the four corners of the bounding rectangle.
//!    - All other corner points must appear an even number of times (they cancel out).
//!
//! ## Algorithm
//! 1. Initialize `min_x`, `min_y` to `i32::MAX` and `max_x`, `max_y` to `i32::MIN`.
//! 2. Initialize `area_sum` to 0 and an empty `HashSet` `corners`.
//! 3. For each rectangle `[x1, y1, x2, y2]`:
//!    - Update global min/max bounds.
//!    - Add its area `(x2 - x1) * (y2 - y1)` to `area_sum`.
//!    - For each of its four corners, toggle presence in `corners`:  
//!      • Insert if not present; remove if already present.
//! 4. Compute `bounding_area = (max_x - min_x) * (max_y - min_y)`.
//! 5. If `area_sum != bounding_area`, return `false` (there is a gap or overlap).
//! 6. Ensure `corners.len() == 4` and that it contains exactly the four bounding corners.
//! 7. Return `true`.
//!
//! ## Time Complexity
//! O(n), where n = number of rectangles.
//!
//! ## Space Complexity
//! O(n), for the corner set.

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// Determines if the given small rectangles together form exactly one perfect rectangle.
    ///
    /// # Arguments
    ///
    /// * `rectangles` - A vector of rectangles, each `[x1, y1, x2, y2]` where
    ///   `(x1, y1)` is the lower-left and `(x2, y2)` is the upper-right corner.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if they form a perfect rectangle without gaps or overlaps.
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        // Initialize bounding extents to extremes.
        let mut min_x: i32 = i32::MAX;
        let mut min_y: i32 = i32::MAX;
        let mut max_x: i32 = i32::MIN;
        let mut max_y: i32 = i32::MIN;

        // Sum of areas of all small rectangles.
        let mut area_sum: i64 = 0;

        // Set to track corner points appearing odd number of times.
        let mut corners: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();

        // Process each small rectangle.
        for rect in rectangles {
            // Destructure corner coordinates.
            let [x1, y1, x2, y2] = rect[..]
                .try_into()
                .expect("Each rectangle must have exactly four integers.");

            // Update global minimum x-coordinate.
            min_x = std::cmp::min(min_x, x1);
            // Update global minimum y-coordinate.
            min_y = std::cmp::min(min_y, y1);
            // Update global maximum x-coordinate.
            max_x = std::cmp::max(max_x, x2);
            // Update global maximum y-coordinate.
            max_y = std::cmp::max(max_y, y2);

            // Add this rectangle's area to the total.
            area_sum += (x2 - x1) as i64 * (y2 - y1) as i64;

            // List of this rectangle's four corners.
            let pts: [(i32, i32); 4] = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];

            // Toggle each corner in the set.
            for &p in &pts {
                if !corners.insert(p) {
                    corners.remove(&p);
                }
            }
        }

        // Compute the area of the bounding rectangle.
        let bounding_area: i64 = (max_x - min_x) as i64 * (max_y - min_y) as i64;

        // If total area differs, there is a gap or overlap.
        if area_sum != bounding_area {
            return false;
        }

        // The set must contain exactly the four corners of the bounding rectangle.
        if corners.len() != 4 {
            return false;
        }

        // The four expected corners.
        let expected: [(i32, i32); 4] = [
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];

        // Verify each expected corner is present.
        for &e in &expected {
            if !corners.contains(&e) {
                return false;
            }
        }

        // All checks passed: perfect rectangle cover.
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_391_single_rectangle() {
        // Single rectangle covers exactly itself.
        let rects = vec![vec![0, 0, 1, 1]];
        assert!(Solution::is_rectangle_cover(rects));
    }

    #[test]
    fn test_391_two_rectangles_perfect_cover() {
        // Two unit squares side by side form a 2×1 rectangle.
        let rects = vec![vec![0, 0, 1, 1], vec![1, 0, 2, 1]];
        assert!(Solution::is_rectangle_cover(rects));
    }

    #[test]
    fn test_391_two_rectangles_gap() {
        // Two squares with a gap in between cannot form a perfect cover.
        let rects = vec![vec![0, 0, 1, 1], vec![2, 0, 3, 1]];
        assert!(!Solution::is_rectangle_cover(rects));
    }

    #[test]
    fn test_391_basic_example_perfect_cover() {
        // Example from problem statement forming a perfect cover.
        let rects = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![3, 2, 4, 4],
            vec![1, 3, 2, 4],
            vec![2, 3, 3, 4],
        ];
        assert!(Solution::is_rectangle_cover(rects));
    }

    #[test]
    fn test_391_example_gap() {
        // Example with a gap should return false.
        let rects = vec![
            vec![1, 1, 2, 3],
            vec![1, 3, 2, 4],
            vec![3, 1, 4, 2],
            vec![2, 1, 3, 2],
        ];
        assert!(!Solution::is_rectangle_cover(rects));
    }

    #[test]
    fn test_391_example_overlap() {
        // Overlapping rectangles cause failure.
        let rects = vec![
            vec![1, 1, 3, 3],
            vec![3, 1, 4, 2],
            vec![1, 3, 2, 4],
            vec![2, 2, 4, 4],
        ];
        assert!(!Solution::is_rectangle_cover(rects));
    }

    #[test]
    fn test_391_negative_coords_perfect_cover() {
        // Perfect cover using rectangles with negative coordinates.
        let rects = vec![
            vec![-1, -1, 1, 1],
            vec![1, -1, 2, 1],
            vec![-1, 1, 1, 2],
            vec![1, 1, 2, 2],
        ];
        assert!(Solution::is_rectangle_cover(rects));
    }

    #[test]
    fn test_391_four_quadrants_perfect_cover() {
        // Four 1×1 squares in each quadrant form a 2×2 square.
        let rects = vec![
            vec![0, 0, 1, 1],
            vec![1, 0, 2, 1],
            vec![0, 1, 1, 2],
            vec![1, 1, 2, 2],
        ];
        assert!(Solution::is_rectangle_cover(rects));
    }
}
