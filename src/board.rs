use std::fmt::Debug;
use funcs::{rand_nanos, xorshift128};

const SIZE: usize = 4;

#[derive(PartialEq, Debug)]
pub struct Board {
    board: [[usize; SIZE]; SIZE],
    board_zeros: usize,
    seed: [u32; 4],
    pub score: usize,
}

impl Board {
    pub fn new() -> Self {
        let seed = [rand_nanos(), rand_nanos(), rand_nanos(), rand_nanos()];
        Board {
            board: [[0; SIZE]; SIZE],
            board_zeros: SIZE * SIZE,
            seed: seed,
            score: 0,
        }
    }
    pub fn up(&mut self) {
        for j in 0..SIZE {
            let mut num_zeros = 0;
            // First pass to count zeros and unite appropriate blocks
            for i in 0..SIZE {
                if self.board[i][j] == 0 {
                    // Increment num zeroes
                    num_zeros += 1;
                } else {
                    // Check if you should combine
                    for k in i + 1..SIZE {
                        // Combine if they're the same and end loop
                        if self.board[i][j] == self.board[k][j] {
                            self.board[i][j] += self.board[k][j];
                            self.board[k][j] = 0;
                            // Every combination increases the number of blanks
                            self.score += self.board[i][j];
                            self.board_zeros += 1;
                            break;
                        } else if self.board[k][j] != 0 {
                            break;
                        }
                    }
                }
            }
            //Second pass to move blocks to edge
            for i in 0..SIZE - num_zeros {
                if self.board[i][j] == 0 {
                    for k in i + 1..SIZE {
                        if self.board[k][j] != 0 {
                            self.board[i][j] = self.board[k][j];
                            self.board[k][j] = 0;
                            break;
                        }
                    }
                }
            }
        }
    }
    pub fn down(&mut self) {
        for j in 0..SIZE {
            let mut num_zeros = 0;
            // First pass to count zeros and unite appropriate blocks
            for i in (0..SIZE).rev() {
                if self.board[i][j] == 0 {
                    // Increment num zeroes
                    num_zeros += 1;
                } else {
                    // Check if you should combine
                    for k in (0..i).rev() {
                        // Combine if they're the same and end loop
                        if self.board[i][j] == self.board[k][j] {
                            self.board[i][j] += self.board[k][j];
                            self.board[k][j] = 0;
                            // Every combination increases the number of blanks
                            self.score += self.board[i][j];
                            self.board_zeros += 1;
                            break;
                        } else if self.board[k][j] != 0 {
                            break;
                        }
                    }
                }
            }
            //Second pass to move blocks to edge
            for i in (num_zeros..SIZE).rev() {
                if self.board[i][j] == 0 {
                    for k in (0..i).rev() {
                        if self.board[k][j] != 0 {
                            self.board[i][j] = self.board[k][j];
                            self.board[k][j] = 0;
                            break;
                        }
                    }
                }
            }
        }
    }
    pub fn left(&mut self) {
        for i in 0..SIZE {
            let mut num_zeros = 0;
            // First pass to count zeros and unite appropriate blocks
            for j in 0..SIZE {
                if self.board[i][j] == 0 {
                    // Increment num zeroes
                    num_zeros += 1;
                } else {
                    // Check if you should combine
                    for k in j + 1..SIZE {
                        // Combine if they're the same and end loop
                        if self.board[i][j] == self.board[i][k] {
                            self.board[i][j] += self.board[i][k];
                            self.board[i][k] = 0;
                            // Every combination increases the number of blanks
                            self.score += self.board[i][j];
                            self.board_zeros += 1;
                            break;
                        } else if self.board[i][k] != 0 {
                            break;
                        }
                    }
                }
            }
            //Second pass to move blocks to edge
            for j in 0..SIZE - num_zeros {
                if self.board[i][j] == 0 {
                    for k in j + 1..SIZE {
                        if self.board[i][k] != 0 {
                            self.board[i].swap(j, k);
                            break;
                        }
                    }
                }
            }
        }
    }
    pub fn right(&mut self) {
        for i in 0..SIZE {
            let mut num_zeros = 0;
            // First pass to count zeros and unite appropriate blocks
            for j in (0..SIZE).rev() {
                if self.board[i][j] == 0 {
                    // Increment num zeroes
                    num_zeros += 1;
                } else if j != 0 {
                    // Check if you should combine
                    for k in (0..j).rev() {
                        // Combine if they're the same and end loop
                        if self.board[i][j] == self.board[i][k] {
                            self.board[i][j] += self.board[i][k];
                            self.board[i][k] = 0;
                            // Every combination increases the number of blanks
                            self.score += self.board[i][j];
                            self.board_zeros += 1;
                            break;
                        } else if self.board[i][k] != 0 {
                            break;
                        }
                    }
                }
            }

            //Second pass to move blocks to edge
            for j in (num_zeros..SIZE).rev() {
                if self.board[i][j] == 0 {
                    for k in (0..j).rev() {
                        if self.board[i][k] != 0 {
                            self.board[i].swap(j, k);
                            break;
                        }
                    }
                }
            }
        }
    }
    pub fn add_rand_block(&mut self) {
        // 1 in 8 odds it will be a 4 instead of a 2
        let num = if xorshift128(&mut self.seed) % 8 == 0 {
            4
        } else {
            2
        };

        let mut pos = xorshift128(&mut self.seed) % self.board_zeros as u32;

        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == 0 {
                    if pos == 0 {
                        self.board[i][j] = num;
                        self.board_zeros -= 1;
                        return;
                    } else {
                        pos -= 1;
                    }
                }
            }
        }
    }
    pub fn print(&self) {
        for i in 0..SIZE {
            println!("{:?}", self.board[i]);
        }
    }
}

#[test]
fn test_up() {
    let mut board = Board::new();
    board.board = [[2, 2, 2, 0], [2, 2, 4, 2], [4, 4, 2, 2], [0, 4, 4, 2]];
    board.board_zeros = 2;
    board.up();

    let mut result_board = Board::new();
    result_board.board = [[4, 4, 2, 4], [4, 8, 4, 2], [0, 0, 2, 0], [0, 0, 4, 0]];
    result_board.board_zeros = 6;

    assert_eq!(result_board.board, board.board);
    assert_eq!(result_board.board_zeros, board.board_zeros);
}

#[test]
fn test_down() {
    let mut board = Board::new();
    board.board = [[0, 4, 4, 2], [4, 4, 2, 2], [2, 2, 4, 2], [2, 2, 2, 0]];
    board.board_zeros = 2;
    board.down();

    let mut result_board = Board::new();
    result_board.board = [[0, 0, 4, 0], [0, 0, 2, 0], [4, 8, 4, 2], [4, 4, 2, 4]];
    result_board.board_zeros = 6;

    assert_eq!(result_board.board, board.board);
    assert_eq!(result_board.board_zeros, board.board_zeros);
}

#[test]
fn test_left() {
    let mut board = Board::new();
    board.board = [[0, 2, 2, 4], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]];
    board.board_zeros = 2;
    board.left();

    let mut result_board = Board::new();
    result_board.board = [[4, 4, 0, 0], [4, 8, 0, 0], [2, 4, 2, 4], [4, 2, 0, 0]];
    result_board.board_zeros = 6;

    assert_eq!(result_board.board, board.board);
    assert_eq!(result_board.board_zeros, board.board_zeros);

    board.board = [[2, 2, 4, 0], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]];
    board.board_zeros = 2;
    board.left();

    result_board.board = [[4, 4, 0, 0], [4, 8, 0, 0], [2, 4, 2, 4], [4, 2, 0, 0]];
    result_board.board_zeros = 6;

    assert_eq!(result_board.board, board.board);
    assert_eq!(result_board.board_zeros, board.board_zeros);
}

#[test]
fn test_right() {
    let mut board = Board::new();
    board.board = [[2, 2, 4, 0], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]];
    board.board_zeros = 2;
    board.right();

    let mut result_board = Board::new();
    result_board.board = [[0, 0, 4, 4], [0, 0, 4, 8], [2, 4, 2, 4], [0, 0, 2, 4]];
    result_board.board_zeros = 6;

    assert_eq!(result_board.board, board.board);
    assert_eq!(result_board.board_zeros, board.board_zeros);

    board.board = [[0, 2, 2, 4], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]];
    board.board_zeros = 2;
    board.right();

    result_board.board = [[0, 0, 4, 4], [0, 0, 4, 8], [2, 4, 2, 4], [0, 0, 2, 4]];
    result_board.board_zeros = 6;

    assert_eq!(result_board.board, board.board);
    assert_eq!(result_board.board_zeros, board.board_zeros);
}
