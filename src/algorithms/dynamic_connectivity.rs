//! This module implements efficient algorithms for dynamic connectivity and percolation.
//!
//! The two core components are:
//! 1. **WeightedQuickUnionUF**: A weighted quick-union data structure with path-halving compression.
//! 2. **Percolation**: A simulation of percolation on an n×n grid using two union-find structures and virtual nodes.
//!
//! # WeightedQuickUnionUF Algorithm
//!
//! - **Initialization (`new(n)`)**:
//!   - Create arrays `parent` and `size` of length `n`.
//!   - For each `i` in `0..n`, set `parent[i] = i` and `size[i] = 1`.
//!   - Time & space: O(n).
//!
//! - **Find root with path-halving (`root(x)`)**:
//!   1. While `x != parent[x]`:
//!      - Set `parent[x] = parent[parent[x]]` (path-halving).
//!      - Update `x = parent[x]`.
//!   2. Return `x`.
//!   - Amortized cost: O(α(n)) where α is inverse Ackermann.
//!
//! - **Union by size (`union(p, q)`)**:
//!   1. Compute `rp = root(p)` and `rq = root(q)`.
//!   2. If `rp == rq`, return (already connected).
//!   3. Attach the smaller tree under the larger:
//!      - If `size[rp] < size[rq]`, set `parent[rp] = rq` and `size[rq] += size[rp]`.
//!      - Else, set `parent[rq] = rp` and `size[rp] += size[rq]`.
//!   - Amortized cost: O(α(n)).
//!
//! - **Connected check (`connected(p, q)`)**:
//!   - Return `root(p) == root(q)` in O(α(n)).
//!
//! # Percolation Algorithm
//!
//! Simulates an n×n grid with two virtual nodes: top (0) and bottom (n*n+1).
//!
//! - **State**:
//!   - `grid: Vec<u8>` of length `n*n+2`, where `0 = blocked`, `1 = open`.
//!   - `uf_perc`: WeightedQuickUnionUF over `n*n+2` elements (percolation check).
//!   - `uf_full`: WeightedQuickUnionUF over `n*n+1` elements (fullness check, no bottom).
//!
//! - **Initialization (`new(n)`)**:
//!   1. Let `total = n*n + 2`.
//!   2. Create `grid = vec![0; total]`, set `grid[0] = 1` (virtual top open).
//!   3. Initialize `uf_perc = WeightedQuickUnionUF::new(total)`.
//!   4. Initialize `uf_full = WeightedQuickUnionUF::new(total - 1)`.
//!   - Time & space: O(n²).
//!
//! - **Index mapping (`idx(row, col)`)**:
//!   - 1-based to linear: `(row-1)*n + col`.
//!
//! - **Open site (`open(row, col)`)**:
//!   1. Bounds-check `row, col ∈ [1..n]`.
//!   2. Compute `i = idx(row,col)`; if `grid[i] == 1`, return.
//!   3. Set `grid[i] = 1`.
//!   4. If `row == 1`, union `0`↔`i` in both `uf_perc` & `uf_full`.
//!   5. If `row == n`, union `i`↔`n*n+1` in `uf_perc`.
//!   6. For each neighbor `(dr,dc) in {(-1,0),(1,0),(0,-1),(0,1)}`:
//!      - If neighbor is within bounds and open, union `i`↔`neighbor` in both UFs.
//!   - Cost per open: O(1) checks + O(α(n²)) unions.
//!
//! - **Queries**:
//!   - `is_open(row,col)`: returns `grid[idx(row,col)] == 1`.
//!   - `is_full(row,col)`: returns `uf_full.connected(0, idx(row,col))`.
//!   - `percolates()`: returns `uf_perc.connected(0, n*n+1)`.
//!   - Each in O(α(n²)).

/// A weighted quick‑union with path‑halving compression.
/// Each element’s parent pointer and tree size are stored in arrays.
pub struct WeightedQuickUnionUF {
    parent: Vec<usize>, // parent[i] = parent of i; if parent[i] == i, i is a root
    size: Vec<usize>,   // size[i] = number of nodes in the tree rooted at i
}

impl WeightedQuickUnionUF {
    /// Initialize `n` singleton sets: each element is its own root, size = 1.
    pub fn new(n: usize) -> Self {
        let mut parent: Vec<usize> = Vec::with_capacity(n);
        let mut size: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            parent.push(i); // point to self
            size.push(1); // tree of size 1
        }

        Self { parent, size }
    }

    /// Find the root of `x`, compressing path by halving:
    /// each node on the path points to its grandparent.
    #[inline]
    fn root(&mut self, mut x: usize) -> usize {
        // Continue until we reach the root (parent[x] == x).

        while x != self.parent[x] {
            // Path‑halving: update x’s parent to its grandparent
            let grandparent: usize = self.parent[self.parent[x]];
            self.parent[x] = grandparent;
            x = grandparent;
        }

        x
    }

    /// Are `p` and `q` in the same component?
    #[inline]
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    /// Merge components containing `p` and `q` by linking smaller tree under larger.
    #[inline]
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.root(p);
        let root_q = self.root(q);
        if root_p == root_q {
            return; // already connected
        }
        // Make smaller root point to larger root
        if self.size[root_p] < self.size[root_q] {
            self.parent[root_p] = root_q; // attach p’s tree under q
            self.size[root_q] += self.size[root_p]; // update size
        } else {
            self.parent[root_q] = root_p; // attach q’s tree under p
            self.size[root_p] += self.size[root_q]; // update size
        }
    }
}

/// Models percolation on an n×n grid with two virtual nodes:
/// - index 0 = virtual top
/// - index n*n+1 = virtual bottom
pub struct Percolation {
    n: usize,                      // grid dimension
    grid: Vec<u8>,                 // 0 = blocked, 1 = open
    uf_perc: WeightedQuickUnionUF, // UF with two virtual nodes for percolation check
    uf_full: WeightedQuickUnionUF, // UF with only the virtual top for fullness check
}

impl Percolation {
    /// Constructs a new percolation grid of size n×n.
    /// All sites start blocked; virtual top is marked open.
    pub fn new(n: usize) -> Self {
        // total sites = n*n + 2 (including two virtual nodes)
        let total: usize = n * n + 2;
        let mut grid: Vec<u8> = vec![0; total];

        grid[0] = 1; // mark virtual top as “open”

        Self {
            n,
            grid,
            uf_perc: WeightedQuickUnionUF::new(total),
            uf_full: WeightedQuickUnionUF::new(total - 1), // omit virtual bottom
        }
    }

    /// Translate 1‑based (row, col) to a linear index in [1..n*n].
    #[inline]
    fn idx(&self, row: usize, col: usize) -> usize {
        // (row-1) * n + col
        (row - 1) * self.n + col
    }

    /// Opens the site at (row, col) if it is not already open.
    /// Then unions it with open neighbors and with virtual nodes as appropriate.
    pub fn open(&mut self, row: usize, col: usize) {
        // Bounds check: row and col must be within [1..n]
        if row == 0 || row > self.n || col == 0 || col > self.n {
            panic!("Indices out of bounds: row={}, col={}", row, col);
        }

        let i: usize = self.idx(row, col);
        if self.grid[i] == 1 {
            return; // already open, nothing to do
        }

        // Mark the site as open
        self.grid[i] = 1;

        // If in top row, connect to virtual top (index 0)
        if row == 1 {
            self.uf_perc.union(0, i);
            self.uf_full.union(0, i);
        }

        // If in bottom row, connect to virtual bottom (index n*n+1) in uf_perc
        if row == self.n {
            let bottom: usize = self.n * self.n + 1;
            self.uf_perc.union(i, bottom);
        }

        // Directions for neighbors: up, down, left, right
        let directions: [(isize, isize); 4] = [(-1isize, 0), (1, 0), (0, -1), (0, 1)];

        for &(dr, dc) in &directions {
            let nr: isize = row as isize + dr;
            let nc: isize = col as isize + dc;

            // Check neighbor bounds
            if nr >= 1 && nr as usize <= self.n && nc >= 1 && nc as usize <= self.n {
                let j: usize = self.idx(nr as usize, nc as usize);

                // If neighbor is open, union in both UF structures
                if self.grid[j] == 1 {
                    self.uf_perc.union(i, j);
                    self.uf_full.union(i, j);
                }
            }
        }
    }

    /// Returns true if site (row, col) is open.
    #[inline]
    pub fn is_open(&self, row: usize, col: usize) -> bool {
        if row == 0 || row > self.n || col == 0 || col > self.n {
            panic!("Indices out of bounds: row={}, col={}", row, col);
        }

        self.grid[self.idx(row, col)] == 1
    }

    /// Returns true if site (row, col) is connected to the top via open sites.
    #[inline]
    pub fn is_full(&mut self, row: usize, col: usize) -> bool {
        if row == 0 || row > self.n || col == 0 || col > self.n {
            panic!("Indices out of bounds: row={}, col={}", row, col);
        }

        // Check connectivity to virtual top in uf_full
        self.uf_full.connected(0, self.idx(row, col))
    }

    /// Returns true if the system percolates (top virtual node connected to bottom).
    #[inline]
    pub fn percolates(&mut self) -> bool {
        let bottom = self.n * self.n + 1;

        // Check connectivity between virtual top (0) and virtual bottom
        self.uf_perc.connected(0, bottom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percolation_1_vertical_percolation() {
        let mut p = Percolation::new(3);
        p.open(1, 2);
        p.open(2, 2);
        p.open(3, 2);
        assert!(p.percolates());
    }

    #[test]
    fn test_percolation_2_no_percolation() {
        let mut p = Percolation::new(3);
        p.open(1, 1);
        p.open(2, 2);
        p.open(3, 3);
        assert!(!p.percolates());
    }

    #[test]
    fn test_percolation_3_fullness_behavior() {
        let mut p = Percolation::new(3);
        p.open(1, 1);
        p.open(2, 1);

        assert!(p.is_full(2, 1));
        assert!(!p.is_full(3, 1));
    }

    #[test]
    fn test_percolation_4_edge_single_cell() {
        let mut p = Percolation::new(1);
        assert!(!p.percolates());

        p.open(1, 1);

        assert!(p.percolates());
        assert!(p.is_full(1, 1));
    }

    #[test]
    fn test_percolation_5_full_grid_percolation() {
        let n = 3;
        let mut p = Percolation::new(n);

        for row in 1..=n {
            for col in 1..=n {
                p.open(row, col);
            }
        }

        assert!(p.percolates());
    }

    #[test]
    fn test_percolation_6_bottom_only_open() {
        let n = 3;
        let mut p = Percolation::new(n);

        for col in 1..=n {
            p.open(n, col);
        }

        assert!(!p.percolates());

        for col in 1..=n {
            assert!(!p.is_full(n, col));
        }
    }

    #[test]
    fn test_percolation_7_top_only_open() {
        let n = 3;
        let mut p = Percolation::new(n);

        for col in 1..=n {
            p.open(1, col);
        }

        assert!(!p.percolates());

        for col in 1..=n {
            assert!(p.is_full(1, col));
        }
    }

    #[test]
    fn test_percolation_8_diagonal_no_percolation() {
        let n = 4;
        let mut p = Percolation::new(n);

        for i in 1..=n {
            p.open(i, i);
        }

        assert!(!p.percolates());
    }

    #[test]
    fn test_percolation_9_backwash_no_fullness() {
        let n = 3;
        let mut p = Percolation::new(n);

        // create vertical path in col 1
        p.open(1, 1);
        p.open(2, 1);
        p.open(3, 1);
        assert!(p.percolates());

        // open a bottom corner not connected to top
        p.open(3, 3);
        assert!(!p.is_full(3, 3));
    }

    #[test]
    fn test_percolation_10_random_pattern() {
        let mut p = Percolation::new(4);
        p.open(1, 2);
        p.open(2, 2);
        p.open(2, 3);
        p.open(3, 3);
        p.open(4, 3);

        assert!(p.percolates());
    }
}
