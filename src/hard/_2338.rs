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
==============================================================================
   Ideal Arrays — Optimised Prime-Exponent Solution
==============================================================================

Perspective
-----------
An “ideal” length-n array is uniquely determined by its final element x
(1 ≤ x ≤ M).  Decompose

        x = ∏ pᵢ^{eᵢ}.

For each prime exponent eᵢ we must distribute eᵢ indistinguishable balls
into n ordered boxes (array positions).  The classical stars-and-bars
count is

        C(n + eᵢ − 1, eᵢ).

Because the prime factors are independent, the number of arrays that end
in x is the product of these binomial coefficients over all eᵢ.  Summing
this product for every x ∈ [1, M] yields the desired total.

Algorithm (asymptotically tight)
--------------------------------
1.  **Linear sieve.**  Compute the smallest prime factor (spf) for every
    integer ≤ M in O(M log log M) ⊂ O(M log M).

2.  **Largest exponent bound.**  A prime power grows as 2^{max_e}; hence
    max_e = ⌊log₂ M⌋ + 1 is sufficient.

3.  **Combinatorial table.**  Pre-compute factorials and modular inverse
    factorials up to max_e.  Build

        comb[e] = C(n + e − 1, e)  (0 ≤ e ≤ max_e).

4.  **Main loop.**  For x = 2..M:
      a. Factor x on the fly via spf, counting each exponent cnt.
      b. Multiply an accumulator by comb[cnt] (mod MOD).
      c. Add the accumulator to the global answer.

    Initialise the answer with 1 to account for x = 1.

5.  Return answer mod 1 000 000 007.

Complexity
----------
Time
  • Sieve:                  Θ(M log log M)
  • Total factorisation:    Θ(M log M)  (harmonic-series bound)
  • Combinatorial table:    Θ(max_e) = Θ(log M)
  • Aggregate:              Θ(M log M)

Space
  • spf array:              Θ(M)
  • O(log M) for factorials, inverses, comb
  • Aggregate:              Θ(M)

This is optimal to leading order under the prime-factor framework: one
must inspect every integer ≤ M at least once.
==============================================================================
*/

/*
============================================================================
  Ideal Arrays — Optimised Prime-Exponent Algorithm
  Detailed line-by-line commentary included.  Code width ≤ 70 columns.
============================================================================
*/

pub struct SolutionOpt;

impl SolutionOpt {
    //───────────────────────────────────────────────────────────────────────
    // Public entry point: counts ideal arrays of length n with elements
    // ≤ max_value.  Returns answer mod 1_000_000_007.
    //───────────────────────────────────────────────────────────────────────
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        // Constant modulus required by the problem statement.
        const MOD: i64 = 1_000_000_007;

        // Convert parameters to usize for indexing.
        let n: usize = n as usize;
        let m: usize = max_value as usize;

        //───────────────────────────────────────────────────────────────
        // 1)  Linear sieve — spf[v] = smallest prime factor of v.
        //───────────────────────────────────────────────────────────────
        let mut spf: Vec<usize> = vec![0; m + 1];

        // Iterate through all integers 2‥=m.
        for i in 2..=m {
            // i is prime if not yet marked.
            if spf[i] == 0 {
                // Mark i's multiples with smallest prime factor i.
                let mut j: usize = i;

                while j <= m {
                    if spf[j] == 0 {
                        spf[j] = i;
                    }

                    j += i;
                }
            }
        }

        //───────────────────────────────────────────────────────────────
        // 2)  Determine maximum exponent any prime can take in [1, m].
        //     A prime ≥ 2 doubled k times gives 2ᵏ ≤ m ⇒ k ≤ log₂ m.
        //───────────────────────────────────────────────────────────────
        let max_e: usize = (m as f64).log2().floor() as usize + 1;

        //───────────────────────────────────────────────────────────────
        // 3)  Pre-compute factorials and inverse factorials mod MOD
        //     up to max_e, then build comb[e] = C(n+e-1, e).
        //───────────────────────────────────────────────────────────────

        // fact[e] = e! mod MOD.
        let mut fact: Vec<i64> = vec![1; max_e + 1];

        for e in 1..=max_e {
            fact[e] = fact[e - 1] * e as i64 % MOD;
        }

        // inv_fact[e] = (e!)⁻¹ mod MOD using Fermat’s little theorem.
        let mut inv_fact: Vec<i64> = vec![1; max_e + 1];
        inv_fact[max_e] = Self::modpow(fact[max_e], MOD - 2, MOD);

        for e in (1..max_e).rev() {
            inv_fact[e] = inv_fact[e + 1] * (e as i64 + 1) % MOD;
        }

        // comb[e] holds C(n+e-1, e) for 0 ≤ e ≤ max_e.
        let mut comb: Vec<i64> = vec![0; max_e + 1];

        // Base case: C(n-1, 0) = 1.
        comb[0] = 1;

        // numerator accumulates Π_{t=0}^{e-1} (n+t).
        let mut numer: i64 = 1;

        for e in 1..=max_e {
            numer = numer * (n + e - 1) as i64 % MOD;
            comb[e] = numer * inv_fact[e] % MOD;
        }

        //───────────────────────────────────────────────────────────────
        // 4)  Main summation over x = 1..m.
        //     Initialise with x = 1 (no primes, contributes 1 array).
        //───────────────────────────────────────────────────────────────
        let mut ans: i64 = 1;

        // Loop over every terminal value x.
        for x in 2..=m {
            // ways accumulates product of comb[exponent] for x.
            let mut ways: i64 = 1;

            // y will be factored destructively.
            let mut y: usize = x;

            // Factor y using spf table.
            while y > 1 {
                // p = smallest prime factor of current y.
                let p: usize = spf[y];

                // cnt = multiplicity of p in y.
                let mut cnt: usize = 0;

                while y % p == 0 {
                    y /= p;
                    cnt += 1;
                }
                // Multiply contribution for this exponent.
                ways = ways * comb[cnt] % MOD;
            }

            // Add contributions of x to global answer.
            ans = (ans + ways) % MOD;
        }

        // Convert 64-bit accumulator back to i32.
        ans as i32
    }

    //───────────────────────────────────────────────────────────────────
    // Compute modular exponentiation base^exp mod m in O(log exp).
    //───────────────────────────────────────────────────────────────────
    fn modpow(mut base: i64, mut exp: i64, m: i64) -> i64 {
        // res starts as multiplicative identity.
        let mut res: i64 = 1;

        // Ensure base ∈ [0, m-1].
        base %= m;

        // Process bits of exp from LSB upward.
        while exp > 0 {
            // If current bit of exp is 1, multiply res by base.
            if exp & 1 == 1 {
                res = (res * base) % m;
            }

            // Square base for next bit.
            base = (base * base) % m;

            // Drop the bit we just handled.
            exp >>= 1;
        }

        // res now equals base^original_exp mod m.
        res
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
        let result: i32 = Solution::ideal_arrays(n, max_value);
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
