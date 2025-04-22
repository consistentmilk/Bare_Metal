pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let n: usize = n as usize;
        let mut res_stack: Vec<String> = Vec::with_capacity(n);

        for i in 0..n {
            let i = i + 1;

            if i % 3 == 0 && i % 5 == 0 {
                res_stack.push(format!("FizzBuzz"));
            } else if i % 3 == 0 {
                res_stack.push(format!("Fizz"));
            } else if i % 5 == 0 {
                res_stack.push(format!("Buzz"));
            } else {
                res_stack.push(format!("{}", i));
            }
        }

        res_stack
    }

    pub fn fizz_buzz_unsafe_optimized(n: i32) -> Vec<String> {
        // Use usize for capacity and indexing
        let n: usize = n as usize;

        // Pre-allocate result Vec for exactly n elements
        let mut result: Vec<String> = Vec::with_capacity(n);

        // SAFETY: we'll write directly into this buffer
        let ptr: *mut String = result.as_mut_ptr();

        // Use counters instead of modulo for speed
        let mut fizz_count: i32 = 3;
        let mut buzz_count: i32 = 5;

        // For each value from 1 to n
        for i in 1..=n {
            // Decrement counters
            fizz_count -= 1;
            buzz_count -= 1;

            // Prepare the string for this slot
            let s: String;

            if fizz_count == 0 && buzz_count == 0 {
                // "FizzBuzz" case: reset both
                fizz_count = 3;
                buzz_count = 5;

                // Build String from literal with exact capacity
                let lit: &str = "FizzBuzz";
                let mut tmp: String = String::with_capacity(lit.len());

                tmp.push_str(lit);
                s = tmp;
            } else if fizz_count == 0 {
                // "Fizz" case: reset fizz counter
                fizz_count = 3;
                let lit: &str = "Fizz";
                let mut tmp: String = String::with_capacity(lit.len());

                tmp.push_str(lit);
                s = tmp;
            } else if buzz_count == 0 {
                // "Buzz" case: reset buzz counter
                buzz_count = 5;
                let lit: &str = "Buzz";
                let mut tmp: String = String::with_capacity(lit.len());

                tmp.push_str(lit);
                s = tmp;
            } else {
                // Numeric case: convert i to decimal string fast
                let mut x: usize = i;

                // Buffer for digits (reversed)
                let mut buf: [u8; 20] = [0u8; 20];
                let mut len: usize = 0;

                // Extract digits in reverse order
                while x > 0 {
                    let d: u8 = (x % 10) as u8;
                    buf[len] = b'0' + d;
                    len += 1;
                    x /= 10;
                }

                // Reverse digits to correct order
                let mid: usize = len / 2;

                for j in 0..mid {
                    buf.swap(j, len - 1 - j);
                }

                // SAFETY: digits are valid UTF-8 ('0'..'9')
                let slice: &str = unsafe { std::str::from_utf8_unchecked(&buf[..len]) };

                // Build String with exact capacity
                let mut tmp: String = String::with_capacity(len);
                tmp.push_str(slice);
                s = tmp;
            }

            // SAFETY: write the constructed String into the Vec buffer
            unsafe {
                std::ptr::write(ptr.add(i - 1), s);
            }
        }

        // SAFETY: all n elements initialized, set Vec length
        unsafe {
            result.set_len(n);
        }

        result
    }
}
