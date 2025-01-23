#![allow(unused)]

pub struct QuickFindUF {
    id: Vec<usize>,
}

impl QuickFindUF {
    pub fn new(n: usize) -> Self {
        let mut id: Vec<usize> = vec![0; n];

        for i in 0..n {
            id[i] = i;
        }

        Self { id }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p] == self.id[q]
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid: usize = self.id[p];
        let qid: usize = self.id[q];

        for i in 0..self.id.len() {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }
    }
}

pub struct QuickUnionUF {
    id: Vec<usize>,
}

impl QuickUnionUF {
    pub fn new(n: usize) -> Self {
        let mut id: Vec<usize> = vec![0; n];

        for i in 0..n {
            id[i] = i;
        }

        Self { id }
    }

    pub fn root(&mut self, mut i: usize) -> usize {
        while i != self.id[i] {
            self.id[i] = self.id[self.id[i]];
            i = self.id[i];
        }

        i
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i: usize = self.root(p);
        let j: usize = self.root(q);

        self.id[i] = j;
    }
}

pub struct Percolation {
    size: usize,
    grid: Vec<bool>,
    union_find: QuickUnionUF,
    weighted_find: QuickUnionUF,
}

impl Percolation {
    pub fn new(n: usize) -> Self {
        let mut grid: Vec<bool> = vec![false; n * n + 2];
        grid[0] = true;

        Self {
            size: n,
            grid,
            union_find: QuickUnionUF::new(n * n + 2),
            weighted_find: QuickUnionUF::new(n * n + 1),
        }
    }

    fn index(&self, i: usize, j: usize) -> usize {
        (i - 1) * self.size + j
    }

    pub fn is_open(&self, i: usize, j: usize) -> bool {
        if i == 0 || i > self.size {
            panic!("row index i out of bounds");
        }

        if j == 0 || j > self.size {
            panic!("col index i out of bounds");
        }

        self.grid[self.index(i, j)]
    }

    pub fn is_full(&mut self, i: usize, j: usize) -> bool {
        if i == 0 || i > self.size {
            panic!("row index i out of bounds");
        }

        if j == 0 || j > self.size {
            panic!("col index i out of bounds");
        }

        self.weighted_find.connected(self.index(i, j), 0)
    }

    pub fn percolates(&mut self) -> bool {
        self.union_find.connected(0, self.size * self.size + 1)
    }

    pub fn open(&mut self, i: usize, j: usize) {
        if i == 0 || i > self.size {
            panic!("row index i out of bounds");
        }

        if j == 0 || j > self.size {
            panic!("col index i out of bounds");
        }

        let serialized_idx = self.index(i, j);
        self.grid[serialized_idx] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dyncon_quickfind() {
        let mut uf: QuickFindUF = QuickFindUF::new(10);
        uf.union(1, 7);
        uf.union(7, 9);

        assert!(uf.connected(1, 9));
        assert!(!uf.connected(2, 4));
        assert!(!uf.connected(1, 3));
    }

    #[test]
    fn test_dyncon_quickunion() {
        let mut uf: QuickUnionUF = QuickUnionUF::new(10);
        uf.union(1, 7);
        uf.union(7, 9);

        assert!(uf.connected(1, 9));
        assert!(!uf.connected(2, 4));
        assert!(!uf.connected(1, 3));
    }
}
