pub struct Solution;

/// Solution for LeetCode 3337: Total Characters in String After Transformations II
///
/// Problem Statement:
/// ------------------
/// You are given:
/// 1. A string `s` of lowercase English letters (length up to 10⁵).
/// 2. An integer `t` (1 ≤ t ≤ 10⁹) denoting how many times to apply a character‐expansion transformation.
/// 3. An array `nums` of length 26, where `nums[i]` (1 ≤ nums[i] ≤ 25) specifies how many
///    consecutive letters each letter `('a' + i)` expands into on each transformation, wrapping
///    around from ‘z’ to ‘a’ as needed.
///
/// Each transformation replaces every character `c` in the current string with the next
/// `nums[c - 'a']` letters in the alphabet (cycling after ‘z’).  After exactly `t` transformations,
/// the resulting string length may be astronomically large, so you must return it modulo 10⁹+7.
///
/// Key Ideas:
/// ----------
/// 1. **Linear Independence of Letters**  
///    Each letter evolves independently.  We only need to know, for a single letter `c`, how many
///    characters it contributes after `t` steps.
///
/// 2. **Recurrence as Matrix Multiplication**  
///    Let `fₖ[c]` = length resulting from starting with letter `c` after `k` transformations.  
///    Then  
///      fₖ[c] = ∑_{i=1..nums[c]} fₖ₋₁[(c+i) mod 26].  
///    This is a linear recurrence in the 26‐dimensional space of letters, which can be encoded
///    as a 26×26 matrix `M`, so that the vector `vₖ` = `[fₖ[0], fₖ[1], …, fₖ[25]]ᵀ` satisfies  
///      vₖ = M · vₖ₋₁  ⇒  vₜ = Mᵗ · v₀,  with v₀ = [1,1,…,1]ᵀ.
///
/// 3. **Fast Exponentiation**  
///    To compute `Mᵗ` for large `t` (up to 10⁹), use binary exponentiation in O(log t) matrix‐multiplications.
///    Each 26×26 multiplication takes O(26³) time, so the total cost is O(26³ · log t), which is
///    perfectly acceptable.
///
/// 4. **Final Length Calculation**  
///    Once `Mᵗ` is computed, the contribution of each original letter `c` in `s` is `fₜ[c]` = sum of
///    row `c` of `Mᵗ`.  Summing `freq[c] * fₜ[c]` over all `c` (where `freq[c]` is the count of `c` in `s`)
///    yields the final length modulo 10⁹+7.
///
/// Implementation Outline:
/// -----------------------
/// 1. **Build** the 26×26 matrix `M` on the stack:  
///      - For each column `c` (letter index), set `M[r][c] = 1` for each `r = (c + i) mod 26`  
///        with `1 ≤ i ≤ nums[c]`, and `0` elsewhere.  
/// 2. **Matrix Exponentiation**  
///      - Implement `mat_mul(A, B)` for 26×26 matrices in O(26³) with modulo arithmetic.  
///      - Implement `mat_pow(M, t)` by binary exponentiation, squaring and multiplying as needed.  
/// 3. **Row Summation**  
///      - Compute an array `f` of length 26 where `f[c] = sum(Mᵗ[c][0..25]) mod 10⁹+7`.  
/// 4. **Frequency Dot Product**  
///      - Count frequencies of each letter in `s` in O(|s|).  
///      - Compute the dot product `∑ freq[c] * f[c] (mod 10⁹+7)` to obtain the answer.
///
/// Complexity Analysis:
/// --------------------
/// - **Time:**  
///     • Building `M`: O(26 · max(nums[c])) = O(26²) worst‐case  
///     • Matrix exponentiation: O(26³ · log t)  
///     • Row summation and dot product: O(26² + |s|)  
///   Overall: O(26³ · log t + |s|).
///
/// - **Space:**  
///     • O(26²) for matrices stored on the stack  
///     • O(26) for frequency and result vectors  
///
/// Parameters:
/// -----------
/// - `s: String` — the initial string of lowercase letters.  
/// - `t: i32` — the number of transformations to apply.  
/// - `nums: Vec<i32>` — mapping from each letter `'a'+i` to its expansion count.
///
/// Returns:
/// --------
/// - `i32` — the length of the string after `t` transformations, modulo 10⁹+7.
impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        // Modulus constant (10^9 + 7) for all arithmetic
        const MOD: i64 = 1_000_000_007;

        // Initialize 26×26 transformation matrix to zeroes
        let mut mat: [[i64; 26]; 26] = [[0; 26]; 26];

        // Build matrix: mat[r][c] = 1 if letter r expands to c
        for r in 0..26 {
            // Number of letters letter r produces each step
            let steps: usize = nums[r] as usize;

            for i in 1..=steps {
                // Compute target letter index with wrap-around
                let c: usize = (r + i) % 26;

                // Mark transformation from r → c
                mat[r][c] = 1;
            }
        }

        // Multiply two 26×26 matrices modulo MOD
        fn mul(a: &[[i64; 26]; 26], b: &[[i64; 26]; 26]) -> [[i64; 26]; 26] {
            // Result matrix initialized to zeros
            let mut res: [[i64; 26]; 26] = [[0; 26]; 26];

            for i in 0..26 {
                for k in 0..26 {
                    // Cache element for efficiency
                    let v: i64 = a[i][k];

                    if v != 0 {
                        for j in 0..26 {
                            // Accumulate product and apply modulus
                            res[i][j] = (res[i][j] + v * b[k][j]) % MOD;
                        }
                    }
                }
            }

            res
        }

        // Fast exponentiation of a 26×26 matrix by exponent exp
        fn pow(mut base: [[i64; 26]; 26], mut exp: u64) -> [[i64; 26]; 26] {
            // Start with identity matrix
            let mut res: [[i64; 26]; 26] = [[0; 26]; 26];

            for i in 0..26 {
                res[i][i] = 1;
            }

            // Binary exponentiation loop
            while exp > 0 {
                if exp & 1 == 1 {
                    // Multiply result by base when low bit is set
                    res = mul(&res, &base);
                }

                // Square the base matrix
                base = mul(&base, &base);

                // Shift exponent right one bit
                exp >>= 1;
            }

            res
        }

        // Compute M^t using fast exponentiation
        let mat_t: [[i64; 26]; 26] = pow(mat, t as u64);

        // Compute f[c] = sum of row c in mat_t (total expansion)
        let mut f: [i64; 26] = [0; 26];
        for c in 0..26 {
            let mut sum: i64 = 0;

            for r in 0..26 {
                // Add each entry, applying modulus
                sum = (sum + mat_t[c][r]) % MOD;
            }

            f[c] = sum;
        }

        // Count frequency of each character in s
        let mut cnt: [i64; 26] = [0; 26];
        for &b in s.as_bytes() {
            // Convert ASCII 'a'..'z' to index 0..25
            let idx = (b - b'a') as usize;

            cnt[idx] += 1;
        }

        // Compute final answer: dot product of cnt and f arrays
        let mut ans: i64 = 0;
        for c in 0..26 {
            ans = (ans + cnt[c] * f[c]) % MOD;
        }

        // Return result as 32-bit integer
        ans as i32
    }
}
