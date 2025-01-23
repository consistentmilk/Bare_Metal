pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut indx = 0;
        let mut direction = Direction::Right;
        let mut sudoku = Sudoku::new(board);
        while indx < 81 {
            match sudoku.tiles[indx] {
                Tile::Const(_) => match direction {
                    Direction::Right => indx += 1,
                    Direction::Left => indx -= 1,
                },
                Tile::Number(_) | Tile::Empty => match sudoku.next_available_number(indx) {
                    Some(n) => {
                        sudoku.set_tile(indx, Tile::Number(n));
                        direction = Direction::Right;
                        indx += 1;
                    }
                    None => {
                        sudoku.set_tile(indx, Tile::Empty);
                        direction = Direction::Left;
                        indx -= 1;
                    }
                },
            }
        }
        sudoku.write_to(board);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Number(u8),
    Const(u8),
}

impl Tile {
    pub fn to_u16(&self) -> u16 {
        match self {
            Tile::Empty => 0,
            Tile::Number(n) | Tile::Const(n) => *n as u16,
        }
    }
}

pub struct Sudoku {
    tiles: [Tile; 81],
    rows: [u16; 9],
    cols: [u16; 9],
    squares: [u16; 9],
}

impl Sudoku {
    pub fn new(tiles: &mut Vec<Vec<char>>) -> Self {
        let mut sudoku = Sudoku {
            tiles: [Tile::Empty; 81],
            rows: [0; 9],
            cols: [0; 9],
            squares: [0; 9],
        };
        tiles
            .iter()
            .flat_map(|row| {
                row.iter().map(|tile| match tile {
                    '.' => Tile::Empty,
                    n => Tile::Const((*n as u32 - '0' as u32) as u8),
                })
            })
            .enumerate()
            .for_each(|(i, tile)| sudoku.set_tile(i, tile));
        sudoku
    }

    pub fn write_to(&self, board: &mut Vec<Vec<char>>) {
        self.tiles
            .iter()
            .enumerate()
            .for_each(|(i, tile)| match tile {
                Tile::Number(n) => {
                    board[i / 9][i % 9] = char::from_u32(*n as u32 + '0' as u32).unwrap()
                }
                _ => (),
            });
    }

    pub fn set_tile(&mut self, indx: usize, tile: Tile) {
        match tile {
            Tile::Empty => {
                self.clear(indx);
                self.tiles[indx] = Tile::Empty;
            }
            Tile::Number(_) | Tile::Const(_) => {
                self.clear(indx);
                self.tiles[indx] = tile;
                let mask = 1 << tile.to_u16();
                self.rows[indx / 9] |= mask;
                self.cols[indx % 9] |= mask;
                self.squares[indx / 27 * 3 + (indx % 9) / 3] |= mask;
            }
        }
    }

    fn clear(&mut self, indx: usize) {
        let mask = !(1 << self.tiles[indx].to_u16());
        if mask != 0b1111111111111110 {
            self.rows[indx / 9] &= mask;
            self.cols[indx % 9] &= mask;
            self.squares[indx / 27 * 3 + (indx % 9) / 3] &= mask;
        }
    }

    fn next_available_number(&self, indx: usize) -> Option<u8> {
        let available = self.avaliable_numbers(indx) | ((1 << (self.tiles[indx].to_u16() + 1)) - 1);
        match available.trailing_ones() {
            n if n <= 9 => Some(n as u8),
            _ => None,
        }
    }

    fn avaliable_numbers(&self, indx: usize) -> u16 {
        let row = indx / 9;
        let col = indx % 9;
        let square = indx / 27 * 3 + (indx % 9) / 3;
        self.rows[row] | self.cols[col] | self.squares[square]
    }
}
