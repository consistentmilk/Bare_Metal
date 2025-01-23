pub struct Solution;

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut a: Vec<Node> = vec![];

        for x in nums {
            a.push(Node {
                ad: x.max(0) as i64,
                ..Default::default()
            });
        }

        let mut st = SegmentTree::from_slice(&a, Node::default(), |x, y| Node {
            ac: (x.ad + y.bc).max(x.ac + y.ac).max(x.ac + y.bc),
            ad: (x.ad + y.bd).max(x.ac + y.ad).max(x.ac + y.bd),
            bc: (x.bd + y.bc).max(x.bc + y.ac).max(x.bc + y.bc),
            bd: (x.bd + y.bd).max(x.bc + y.ad).max(x.bc + y.bd),
        });

        let mut ans: i64 = 0;
        for q in queries {
            let (pos, x) = (q[0] as usize, q[1]);
            st.set(
                pos,
                Node {
                    ad: x.max(0) as i64,
                    ..Default::default()
                },
            );

            let node: Node = st.root();
            ans += node.ac.max(node.ad).max(node.bc).max(node.bd);
            ans %= 1_000_000_007;
        }

        ans as i32
    }
}

#[derive(Clone, Default)]
struct Node {
    ac: i64,
    ad: i64,
    bc: i64,
    bd: i64,
}

pub struct SegmentTree<T, F> {
    n: usize,
    p: usize,
    a: Vec<T>,
    t: T,
    op: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, t: T, op: F) -> Self {
        let p: usize = n.next_power_of_two();
        let a: Vec<T> = vec![t.clone(); 2 * p];
        SegmentTree { n, p, a, t, op }
    }

    pub fn from_slice(src: &[T], t: T, op: F) -> Self {
        let n: usize = src.len();
        let p: usize = n.next_power_of_two();
        let mut a: Vec<T> = vec![t.clone(); 2 * p];
        a[p..p + n].clone_from_slice(src);

        for i in (1..p).rev() {
            a[i] = op(&a[2 * i], &a[2 * i + 1]);
        }

        SegmentTree { n, p, a, t, op }
    }

    pub fn root(&self) -> T {
        self.a[1].clone()
    }

    pub fn get(&self, index: usize) -> T {
        self.a[index + self.p].clone()
    }

    pub fn set(&mut self, index: usize, value: T) {
        let mut i: usize = index + self.p;
        self.a[i] = value;
        i >>= 1;
        while i > 0 {
            self.a[i] = (self.op)(&self.a[2 * i], &self.a[2 * i + 1]);
            i >>= 1;
        }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        let mut l: usize = l + self.p;
        let mut r: usize = r + self.p;
        let mut x: T = self.t.clone();
        let mut y: T = self.t.clone();
        while l < r {
            if l % 2 != 0 {
                x = (self.op)(&x, &self.a[l]);
                l += 1;
            }

            if r % 2 != 0 {
                r -= 1;
                y = (self.op)(&self.a[r], &y);
            }

            l >>= 1;
            r >>= 1;
        }

        (self.op)(&x, &y)
    }

    pub fn search_right(&self, l: usize, mut f: impl FnMut(&T) -> bool) -> usize {
        if l == self.n {
            return self.n;
        }
        let mut index: usize = l + self.p;
        let mut cur: T = self.t.clone();

        loop {
            while index % 2 == 0 {
                index >>= 1;
            }

            let next: T = (self.op)(&cur, &self.a[index]);

            if f(&next) {
                cur = next;
                index += 1;

                if index.is_power_of_two() {
                    return self.n;
                }
            } else {
                while index < self.p {
                    index <<= 1;
                    let next = (self.op)(&cur, &self.a[index]);

                    if f(&next) {
                        cur = next;
                        index += 1;
                    }
                }
                return index - self.p;
            }
        }
    }

    pub fn search_left(&self, r: usize, mut f: impl FnMut(&T) -> bool) -> usize {
        if r == 0 {
            return 0;
        }

        let mut index: usize = r + self.p;
        let mut cur: T = self.t.clone();
        loop {
            index -= 1;

            while index > 1 && index % 2 == 1 {
                index >>= 1;
            }

            let next: T = (self.op)(&self.a[index], &cur);

            if f(&next) {
                cur = next;

                if index.is_power_of_two() {
                    return 0;
                }
            } else {
                while index < self.p {
                    index <<= 1;
                    index += 1;
                    let next: T = (self.op)(&self.a[index], &cur);

                    if f(&cur) {
                        cur = next;
                        index -= 1;
                    }
                }

                return index + 1 - self.p;
            }
        }
    }
}
