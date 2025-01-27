1-27-2025 -> Moved to nightly, allocator_api, miri tests and usage of unstable features

We want to run commands from the root, because of the utils module. Various problems related to Linked Lists and Binary Search Trees depend on the structs and macros provided by this module.

Root CMD commands:
-> cargo test test_110
Only tests the tests written for the problem source \_110.rs

TODO:
Rewrite unit tests to use their problem number so that we can filter them while running cargo test

Example:

\_001.rs

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }
}

```

Should be rewritten to:

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_001_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn test_001_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }
}

```

Then we will be able to run only these two tests using the command: cargo test test_001
