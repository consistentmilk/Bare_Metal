use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut product_count: HashMap<i32, i32> = HashMap::new();
        let mut total_number_of_tuples = 0;

        // Precompute all possible products of pairs (nums[i], nums[j]) where i < j
        for i in 0..n {
            for j in i + 1..n {
                let product = nums[i] * nums[j];
                *product_count.entry(product).or_insert(0) += 1;
            }
        }

        // Calculate the total number of tuples
        for &count in product_count.values() {
            if count >= 2 {
                // For each product with frequency `count`, the number of tuples is:
                // count * (count - 1) * 4
                total_number_of_tuples += count * (count - 1) * 4;
            }
        }

        total_number_of_tuples
    }
}

pub struct AlgorithmOne;
// Algorithm 1: Time Complexity: O(n^3) | Space Complexity: O(n)
// 1. Initialize numsLength to the length of the nums array.
// 2. Sort the array in increasing order.
// 3. Initialize totalNumberOfTuples to 0.
// 4. Iterate over nums to try out all possible values of a with aIndex from 0 to numsLength - 1.
//      - Iterate over the rest values of nums to try all possible values for b with bIndex from numsLength - 1 to aIndex + 1.
//          - Define product as nums[aIndex] * nums[bIndex].
//          - Initialize a hash map possibleDValues.
//              - Iterate over nums with cIndex from aIndex + 1 to bIndex - 1:
//                  - If the condition can be satisfied for some integer value of d, i.e. if product % nums[cIndex] == 0:
//                      - Define the desired value of d as dValue = product / nums[cIndex].
//                      - If dValue is in possibleDValues then add 8 (all possible tuples) to totalNumberOfTuples.
//                      - Add nums[cindex] to the possibleDValues.
// 5. Return totalNumberOfTuples.
impl AlgorithmOne {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let nums_length: usize = nums.len();
        let mut nums: Vec<i32> = nums;
        nums.sort_unstable(); // Step 2: Sort the array in increasing order

        let mut total_number_of_tuples: i32 = 0; // Step 3: Initialize totalNumberOfTuples to 0

        // Step 4: Iterate over nums to try all possible values of a
        for a_index in 0..nums_length {
            // Iterate over the rest values of nums to try all possible values for b
            for b_index in (a_index + 1..nums_length).rev() {
                let product: i32 = nums[a_index] * nums[b_index]; // Define product as nums[aIndex] * nums[bIndex]

                let mut possible_d_values: HashMap<i32, bool> = HashMap::new(); // Initialize a hash map possibleDValues

                // Iterate over nums with cIndex from aIndex + 1 to bIndex - 1
                for c_index in a_index + 1..b_index {
                    // If the condition can be satisfied for some integer value of d
                    if product % nums[c_index] == 0 {
                        let d_value: i32 = product / nums[c_index]; // Define the desired value of d

                        // If dValue is in possibleDValues, add 8 to totalNumberOfTuples
                        if possible_d_values.contains_key(&d_value) {
                            total_number_of_tuples += 8;
                        }

                        // Add nums[c_index] to the possibleDValues
                        possible_d_values.insert(nums[c_index], true);
                    }
                }
            }
        }

        total_number_of_tuples // Step 5: Return totalNumberOfTuples
    }
}

pub struct AlgorithmTwo;
// Algorithm 2: Time Complexity O((n^2)*log(n)) | Space Complexity: O(n^2)
// 1. Initialize
//      - numsLength to the length of the nums array.
//      - an array, pairProducts, to store the pairwise products of the elements.
//      - totalNumberOfTuples to 0.
// 2. Iterate over nums with firstIndex from 0 to numsLength - 1:
//      - Iterate over nums with secondindex from firstIndex + 1 to numsLength - 1:
//         - Add the product nums[firstIndex] * nums[secondindex] to the pairProducts list.
// 3. Sort pairProducts in increasing order.
// 4. Initialize lastProductSeen to -1 and sameProductCount to 0.
// 5. Iterate over pairProducts with productIndex from 0 to pairProducts.size - 1:
//      - If the current product is equal to the last seen:
//          - Increment sameProductCount by 1.
//      - Otherwise:
//          -- Calculate the number of pairs of products with value lastProductSeen: pairsOfEqualProduct = (sameProductCount - 1) * sameProductCount / 2.
//      - Add all possible tuples for that product value to the total: increment totalNumberOfTuples by 8 * pairsOfEqualProduct.
//      - Set lastProductSeen to the pairProducts[productIndex] and sameProductCount to 1.
// 6. Handle the last group of products:
//      - Calculate the number of pairs of products with value lastProductSeen: pairsOfEqualProduct = (sameProductCount - 1) * sameProductCount / 2.
//      - Add all possible tuples for that product value to the total: increment totalNumberOfTuples by 8 * pairsOfEqualProduct.
// 7. Return totalNumberOfTuples.
impl AlgorithmTwo {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let nums_length = nums.len();

        // Step 1: Initialize pairProducts and totalNumberOfTuples
        let mut pair_products = Vec::new();
        let mut total_number_of_tuples = 0;

        // Step 2: Compute all pairwise products
        for first_index in 0..nums_length {
            for second_index in first_index + 1..nums_length {
                pair_products.push(nums[first_index] * nums[second_index]);
            }
        }

        // Step 3: Sort pairProducts in increasing order
        pair_products.sort_unstable();

        // Step 4: Initialize lastProductSeen and sameProductCount
        let mut last_product_seen = -1;
        let mut same_product_count = 0;

        // Step 5: Iterate over pairProducts
        for &product in &pair_products {
            if product == last_product_seen {
                // Increment sameProductCount if the product is the same as the last seen
                same_product_count += 1;
            } else {
                // Calculate the number of pairs of products with value lastProductSeen
                if same_product_count >= 2 {
                    let pairs_of_equal_product = (same_product_count - 1) * same_product_count / 2;
                    // Add all possible tuples for that product value to the total
                    total_number_of_tuples += 8 * pairs_of_equal_product;
                }

                // Reset for the new product
                last_product_seen = product;
                same_product_count = 1;
            }
        }

        // Step 6: Handle the last group of products
        if same_product_count >= 2 {
            let pairs_of_equal_product = (same_product_count - 1) * same_product_count / 2;
            total_number_of_tuples += 8 * pairs_of_equal_product;
        }

        // Step 7: Return totalNumberOfTuples
        total_number_of_tuples
    }
}

pub struct AlgorithmThree;
// Algorithm 3: Time Complexity: O(n^2) | Space Complexity: O(n^2)
// 1. Initialize
//      - numsLength to the length of the nums array.
//      - a hash map, pairProductsFrequency.
//      - totalNumberOfTuples to 0.
// 2. Iterate over nums with firstIndex from 0 to numsLength - 1:
//      - Iterate over nums with secondindex from firstIndex + 1 to numsLength - 1:
//          - Increment the frequency of the product: nums[firstIndex] * nums[secondindex] by 1.
// 3. For each element [productValue, productFrequency] of pairProductsFrequency:
//      - Calculate the number of pairs of products with value productValue: pairsOfEqualProduct = (productFrequency - 1) * productFrequency / 2.
//      - Add all possible tuples for that product value to the total: increment totalNumberOfTuples by 8 * pairsOfEqualProduct.
// 4. Return totalNumberOfTuples.
impl AlgorithmThree {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut count: HashMap<i32, i32> = HashMap::with_capacity((n * (n - 1)) >> 1);

        for i in 0..n {
            for j in (i + 1)..n {
                let prod: i32 = nums[i] * nums[j];
                *count.entry(prod).or_insert(0) += 1;
            }
        }

        let mut result: i32 = 0;
        for &freq in count.values() {
            if freq > 1 {
                result += 4 * freq * (freq - 1);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1726_1() {
        let nums: Vec<i32> = vec![2, 3, 4, 6];
        let expected: i32 = 8;
        assert_eq!(expected, Solution::tuple_same_product(nums));
    }

    #[test]
    fn test_1726_2() {
        let nums: Vec<i32> = vec![1, 2, 4, 5, 10];
        let expected: i32 = 16;
        assert_eq!(expected, Solution::tuple_same_product(nums));
    }
}
