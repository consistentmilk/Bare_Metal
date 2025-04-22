/*
Leetcode 2338 — Count the Number of Ideal Arrays

------------------------------------------------
Intuition:
------------------------------------------------
We are asked to count arrays `arr` of length `n` such that:
  - Each arr[i] is in [1, maxValue]
  - Each arr[i] is divisible by arr[i-1]

This structure resembles *chains* where each number divides
the next: v₁ → v₂ → … → vₖ, which we call *divisor chains*.

To compute the total number of such arrays:
  1. Enumerate all valid *distinct* divisor chains of length k
  2. Count the ways to *stretch* each such chain to length `n`
     using repeated elements

The number of stretchings is C(n-1, k-1), which counts how to
partition n slots into k ≥1 ordered blocks, each ≥1 element.

------------------------------------------------
Algorithm:
------------------------------------------------
1. Precompute proper divisors for all numbers up to maxValue
2. Use dynamic programming to compute:
     dp[k][v] = # of length-k chains ending at value `v`
3. Compute total number of chains for each `k`: f[k]
4. Precompute factorials and inverse factorials for nCk
5. Accumulate total arrays as:
     Σ_{k=1}^{k_max} f[k] * C(n-1, k-1)

------------------------------------------------
Time Complexity:
------------------------------------------------
- Divisor sieve: O(M log M)
- DP fill: O(K * M * avg_divs)
- Factorials: O(n)
- Overall: O(M log M + n)

------------------------------------------------
Space Complexity:
------------------------------------------------
- O(K * M) for dp table
- O(n) for factorials
- O(M) for divisor list
*/

pub struct Solution;

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Convert inputs to usize for indexing
        let n: usize = n as usize;
        let max_val: usize = max_value as usize;

        // Step 1: Precompute all divisors up to max_val
        let mut divisors: Vec<Vec<usize>> = vec![vec![]; max_val + 1];

        for d in 1..=max_val {
            let mut m: usize = d;

            while m <= max_val {
                divisors[m].push(d);
                m += d;
            }
        }

        // Step 2: Determine max chain length ≈ log₂(max_val)
        let mut k_max: usize = 0;
        let mut p: usize = 1;

        while p <= max_val {
            k_max += 1;
            p <<= 1;
        }

        // Step 3: Initialize DP table
        let mut dp: Vec<Vec<i64>> = vec![vec![0; max_val + 1]; k_max + 1];

        // Base case: dp[1][v] = 1 for any v
        for v in 1..=max_val {
            dp[1][v] = 1;
        }

        // Fill dp[k][v] for all k ≥ 2 and v ∈ [1, max_val]
        for k in 2..=k_max {
            for v in 1..=max_val {
                let mut total: i64 = 0;

                // Sum over all proper divisors of v
                for &d in &divisors[v] {
                    if d < v {
                        total = (total + dp[k - 1][d]) % MOD;
                    }
                }

                dp[k][v] = total;
            }
        }

        // Step 4: Compute total number of chains of each length
        let mut f: Vec<i64> = vec![0i64; k_max + 1];
        for k in 1..=k_max {
            for v in 1..=max_val {
                f[k] = (f[k] + dp[k][v]) % MOD;
            }
        }

        // Step 5: Precompute factorials and inv factorials
        let mut fact: Vec<i64> = vec![1i64; n + 1];
        let mut inv_fact: Vec<i64> = vec![1i64; n + 1];

        for i in 1..=n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }

        inv_fact[n] = Self::modinv(fact[n], MOD);

        for i in (1..n).rev() {
            inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % MOD;
        }

        // Helper closure to compute nCk mod MOD
        let comb = |nn: usize, kk: usize| -> i64 {
            if kk > nn {
                0
            } else {
                fact[nn] * inv_fact[kk] % MOD * inv_fact[nn - kk] % MOD
            }
        };

        // Step 6: Sum f[k] * C(n-1, k-1) over valid k
        let mut total: i64 = 0i64;
        for k in 1..=k_max {
            if k > n {
                break;
            }

            let ways: i64 = comb(n - 1, k - 1);
            total = (total + f[k] * ways) % MOD;
        }

        total as i32
    }

    // Compute modular inverse using Fermat's theorem
    fn modinv(a: i64, m: i64) -> i64 {
        Self::modpow(a, m - 2, m)
    }

    // Efficient binary exponentiation: a^b % m
    fn modpow(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut result: i64 = 1;
        base %= m;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % m;
            }

            base = base * base % m;
            exp /= 2;
        }

        result
    }
}

/*
Intuition:
  Every ideal array of length n and max_value M corresponds to
  selecting, for each integer x in [1..M], how its prime exponents
  distribute across the n positions. If x = ∏ pᵢ^eᵢ, each exponent eᵢ
  can be split into n nonnegative parts in C(n + eᵢ - 1, eᵢ) ways.
  Summing these products over all x yields the total count.

Algorithm:
  1. Build a sieve of minimum prime factors up to M.
  2. Estimate max_e = ⌊log₂ M⌋ + 1 for the largest exponent.
  3. Precompute factorials and inv_factorials up to max_e.
  4. Build comb[e] = C(n + e - 1, e) for e in 0..=max_e.
  5. For each x from 2 to M:
       a. Factor x on the fly via the sieve, counting each prime’s
          exponent cnt.
       b. Multiply ways by comb[cnt] mod MOD.
       c. Add ways to the global answer.
     Include x=1’s contribution (1).
  6. Return answer % MOD.

Time Complexity:
  • Sieve: O(M log M)
  • On‑the‑fly factorization: O(M log M) total
  • Combination precompute: O(max_e)
  Overall: O(M log M)

Space Complexity:
  • O(M) for the sieve array
  • O(max_e) for factorials, inv_factorials, comb
  Total: O(M + log M)
*/

pub struct SolutionOpt;

impl SolutionOpt {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        // Use 64-bit accumulator to prevent overflow before modulo
        const MOD: i64 = 1_000_000_007;

        // Convert inputs to usize for indexing into Vecs
        let n: usize = n as usize;
        let m: usize = max_value as usize;

        // -------------------------------
        // 1) Build sieve of minimum primes
        // -------------------------------

        // Allocate array of size m+1, initialized to 0
        // min_prime[x] will hold the smallest prime divisor of x
        let mut min_prime: Vec<usize> = vec![0usize; m + 1];

        // Iterate from 2 up to m to populate sieve
        for i in 2..=m {
            // If min_prime[i] is 0, i is prime
            if min_prime[i] == 0 {
                // Mark i as its own smallest prime factor
                let mut j: usize = i;

                // For each multiple of i ≤ m
                while j <= m {
                    // If not already marked, set min_prime[j] = i
                    if min_prime[j] == 0 {
                        min_prime[j] = i;
                    }

                    j += i;
                }
            }
        }

        // --------------------------------------------------------
        // 2) Determine maximum exponent needed: max_e ≈ log₂(m)
        // --------------------------------------------------------

        // Estimate largest exponent any prime can have in [1..m]
        let max_e: usize = (m as f64).log2().floor() as usize + 1;

        // --------------------------------------------------------
        // 3) Precompute factorials & inverse factorials up to max_e
        // --------------------------------------------------------

        // fact_e[e] = e! mod MOD, for e from 0..=max_e
        let mut fact_e: Vec<i64> = vec![1i64; max_e + 1];

        for e in 1..=max_e {
            // Multiply previous factorial by e
            fact_e[e] = fact_e[e - 1] * e as i64 % MOD;
        }

        // inv_fact_e[e] = (e!)⁻¹ mod MOD, computed via Fermat
        let mut inv_fact_e: Vec<i64> = vec![1i64; max_e + 1];

        // Compute inverse of max_e! using modular exponentiation
        inv_fact_e[max_e] = Self::modinv(fact_e[max_e], MOD);

        // Fill inv_fact in descending order using inv_fact[i] =
        // inv_fact[i+1] * (i+1) mod MOD
        for e in (1..max_e).rev() {
            inv_fact_e[e] = inv_fact_e[e + 1] * (e as i64 + 1) % MOD;
        }

        // --------------------------------------------------------
        // 4) Build comb[e] = C(n+e-1, e) for e = 0..=max_e
        // --------------------------------------------------------

        // comb[e] stores the number of ways to distribute exponent e
        // across n positions: C(n+e-1, e)
        let mut comb: Vec<i64> = vec![0i64; max_e + 1];

        // Base case: C(n-1, 0) = 1
        comb[0] = 1;

        // numer accumulates product (n)*(n+1)*...*(n+e-1)
        let mut numer: i64 = 1i64;

        for e in 1..=max_e {
            // Multiply by the next term in numerator sequence
            numer = numer * (n + e - 1) as i64 % MOD;

            // Divide by e! via multiplying inverse factorial
            comb[e] = numer * inv_fact_e[e] % MOD;
        }

        // --------------------------------------------------------
        // 5) Sum contributions for each x = 1..m
        // --------------------------------------------------------

        // Initialize answer with x=1’s contribution (no primes)
        let mut ans: i64 = 1i64;

        // For x from 2 to m, factor and accumulate ways
        for x in 2..=m {
            // Start with 1 way for this x
            let mut ways: i64 = 1i64;
            let mut y: usize = x;

            // Factor y by repeatedly extracting min_prime[y]
            while y > 1 {
                // p = smallest prime divisor of y
                let p: usize = min_prime[y];

                // Count how many times p divides y
                let mut cnt: usize = 0;

                while y % p == 0 {
                    y /= p;
                    cnt += 1;
                }

                // Multiply ways by comb[cnt], handling exponent cnt
                ways = ways * comb[cnt] % MOD;
            }

            // Add this x’s total ways to the global answer
            ans = (ans + ways) % MOD;
        }

        // Return final result as 32-bit integer
        ans as i32
    }

    // Compute modular inverse of a under modulus m via Fermat’s theorem
    fn modinv(a: i64, m: i64) -> i64 {
        Self::modpow(a, m - 2, m)
    }

    // Fast exponentiation: compute (base^exp) % m in O(log exp)
    fn modpow(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut result: i64 = 1i64;

        // 1. Reduce base modulo m so it starts in [0..m‑1]
        base %= m;

        // 2. Loop as long as there are bits left in exp
        while exp > 0 {
            // 2a. If the current least‑significant bit of exp is 1,
            //     multiply it into our accumulating result.
            //     This accounts for that “power-of-two” factor.
            if exp & 1 == 1 {
                result = (result * base) % m;
            }

            // 2b. Square the base: we’re moving to the next bit,
            //     so base ← base² mod m. This prepares the factor
            //     for the next higher bit in exp.
            base = (base * base) % m;

            // 2c. Shift exp right by 1 bit, discarding the bit we just processed.
            exp >>= 1;
        }

        // 3. Once exp hits zero, we have multiplied in exactly those
        //    powers of base corresponding to the 1‑bits of the original exp.
        //    `result` now equals (original_base^original_exp) mod m.
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // test_2338_1_n2_max5: n=2, maxValue=5 →10
    fn test_2338_1_n2_max5() {
        // input length
        let n: i32 = 2;
        // input max value
        let max_value: i32 = 5;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 10;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_2_n5_max3: n=5, maxValue=3 →11
    fn test_2338_2_n5_max3() {
        // input length
        let n: i32 = 5;
        // input max value
        let max_value: i32 = 3;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 11;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_3_n2_max1: n=2, maxValue=1 →1
    fn test_2338_3_n2_max1() {
        // input length
        let n: i32 = 2;
        // input max value
        let max_value: i32 = 1;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 1;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_4_n3_max2: n=3, maxValue=2 →4
    fn test_2338_4_n3_max2() {
        // input length
        let n: i32 = 3;
        // input max value
        let max_value: i32 = 2;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 4;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_5_n3_max3: n=3, maxValue=3 →7
    fn test_2338_5_n3_max3() {
        // input length
        let n: i32 = 3;
        // input max value
        let max_value: i32 = 3;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 7;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_6_n4_max4: n=4, maxValue=4 →19
    fn test_2338_6_n4_max4() {
        // input length
        let n: i32 = 4;
        // input max value
        let max_value: i32 = 4;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 19;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_7_n10_max1: n=10, maxValue=1 →1
    fn test_2338_7_n10_max1() {
        // input length
        let n: i32 = 10;
        // input max value
        let max_value: i32 = 1;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 1;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_8_n3_max4: n=3, maxValue=4 →13
    fn test_2338_8_n3_max4() {
        // input length
        let n: i32 = 3;
        // input max value
        let max_value: i32 = 4;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 13;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_9_n5_max5: n=5, maxValue=5 →31
    fn test_2338_9_n5_max5() {
        // input length
        let n: i32 = 5;
        // input max value
        let max_value: i32 = 5;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 31;
        assert_eq!(result, expected);
    }

    #[test]
    // test_2338_10_n4_max5: n=4, maxValue=5 →23
    fn test_2338_10_n4_max5() {
        // input length
        let n: i32 = 4;
        // input max value
        let max_value: i32 = 5;
        // execute solution
        let result = Solution::ideal_arrays(n, max_value);
        // expected result
        let expected: i32 = 23;
        assert_eq!(result, expected);
    }
}
