pub struct NumberTheory;

impl NumberTheory {
    pub fn gcd(a: u128, b: u128) -> u128 {
        let mut a: u128 = a.clone();
        let mut b: u128 = b.clone();

        while b != 0 {
            let temp: u128 = b;
            b = a % b;
            a = temp;
        }

        a
    }

    pub fn lcm(a: u128, b: u128) -> u128 {
        (a * b) / Self::gcd(a, b)
    }

    pub fn nth_prime(n: u128) -> u128 {
        let mut count: u128 = 0;
        let mut num: u128 = 2;

        while count < n {
            if Self::is_prime(num) {
                count += 1;
            }

            num += 1;
        }

        num - 1
    }

    pub fn is_prime(num: u128) -> bool {
        if num <= 1 {
            return false;
        }

        let limit: u128 = (num as f64).sqrt() as u128;

        for i in 2..=limit {
            if num % i == 0 {
                return false;
            }
        }

        true
    }

    pub fn find_pythagorean_triplet(sum: i128) -> (i128, i128, i128) {
        for a in 1..sum {
            for b in a..sum {
                let c: i128 = sum - a - b;

                if a.pow(2) + b.pow(2) == c.pow(2) {
                    return (a, b, c);
                }
            }
        }

        (0, 0, 0)
    }

    pub fn sum_of_primes_below(limit: usize) -> usize {
        let mut sieve: Vec<bool> = vec![true; limit];
        sieve[0] = false;
        sieve[1] = false;

        for i in 2..=(limit as f64).sqrt() as usize {
            if sieve[i] {
                let mut j: usize = i * i;

                while j < limit {
                    sieve[j] = false;
                    j += i;
                }
            }
        }

        let mut sum: usize = 0;

        for (i, &is_prime) in sieve.iter().enumerate() {
            if is_prime {
                sum += i;
            }
        }

        sum
    }
}
