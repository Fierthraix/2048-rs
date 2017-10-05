use std::fmt::Debug;

const SIZE: usize = 4;

#[derive(PartialEq, Debug)]
pub struct Board {
    board: [[usize; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        //TODO; add some random pieces to start
        Board { board: [[0; SIZE]; SIZE] }
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
    pub fn add_rand_blocks(&mut self) {}
}

#[test]
fn test_up() {
    let mut board = Board { board: [[2, 2, 2, 0], [2, 2, 4, 2], [4, 4, 2, 2], [0, 4, 4, 2]] };
    board.up();

    let result_board = Board { board: [[4, 4, 2, 4], [4, 8, 4, 2], [0, 0, 2, 0], [0, 0, 4, 0]] };

    assert_eq!(result_board, board);
}

#[test]
fn test_down() {
    let mut board = Board { board: [[0, 4, 4, 2], [4, 4, 2, 2], [2, 2, 4, 2], [2, 2, 2, 0]] };
    board.down();

    let result_board = Board { board: [[0, 0, 4, 0], [0, 0, 2, 0], [4, 8, 4, 2], [4, 4, 2, 4]] };

    assert_eq!(result_board, board);
}

#[test]
fn test_left() {
    let mut board = Board { board: [[0, 2, 2, 4], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]] };
    board.left();

    let result_board = Board { board: [[4, 4, 0, 0], [4, 8, 0, 0], [2, 4, 2, 4], [4, 2, 0, 0]] };

    assert_eq!(result_board, board);

    let mut board = Board { board: [[2, 2, 4, 0], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]] };
    board.left();

    let result_board = Board { board: [[4, 4, 0, 0], [4, 8, 0, 0], [2, 4, 2, 4], [4, 2, 0, 0]] };
}

#[test]
fn test_right() {
    let mut board = Board { board: [[2, 2, 4, 0], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]] };
    board.right();

    let result_board = Board { board: [[0, 0, 4, 4], [0, 0, 4, 8], [2, 4, 2, 4], [0, 0, 2, 4]] };

    assert_eq!(result_board, board);

    let mut board = Board { board: [[0, 2, 2, 4], [2, 2, 4, 4], [2, 4, 2, 4], [0, 2, 2, 2]] };
    board.right();

    let result_board = Board { board: [[0, 0, 4, 4], [0, 0, 4, 8], [2, 4, 2, 4], [0, 0, 2, 4]] };

    assert_eq!(result_board, board);
}
