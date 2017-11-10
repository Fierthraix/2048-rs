pub const SIZE: usize = 4;

#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    board: [[usize; SIZE]; SIZE],
    board_zeros: usize,
    seed: [u32; 4],
    score: usize,
}

impl Board {
    pub fn new() -> Self {
        let seed = [rand_nanos(), rand_nanos(), rand_nanos(), rand_nanos()];
        let mut b = Board {
            board: [[0; SIZE]; SIZE],
            board_zeros: SIZE * SIZE,
            seed: seed,
            score: 0,
        };
        b.add_rand_block();
        b
    }
    pub fn up(&mut self) {
        let mut movement = false;
        for j in 0..SIZE {
            let mut row = [0; SIZE];
            let mut k = 0;
            for i in 0..SIZE {
                row[k] = self.board[i][j];
                k += 1;
            }
            if self.move_row(&mut row) {
                movement = true;
                let mut k = 0;
                for i in 0..SIZE {
                    self.board[i][j] = row[k];
                    k += 1;
                }
            }
        }
        if movement {
            self.add_rand_block();
        }
    }
    pub fn down(&mut self) {
        let mut movement = false;
        for j in 0..SIZE {
            let mut row = [0; SIZE];
            let mut k = 0;
            for i in (0..SIZE).rev() {
                row[k] = self.board[i][j];
                k += 1;
            }
            if self.move_row(&mut row) {
                movement = true;
                let mut k = 0;
                for i in (0..SIZE).rev() {
                    self.board[i][j] = row[k];
                    k += 1
                }
            }
        }
        if movement {
            self.add_rand_block();
        }
    }
    pub fn left(&mut self) {
        let mut movement = false;
        for i in 0..SIZE {
            // Get the row
            let mut row = self.board[i];
            // Move it and respond if it moved or not
            if self.move_row(&mut row) {
                movement = true;
                self.board[i] = row;
            }
        }
        // If the board changed add a new block
        if movement {
            self.add_rand_block();
        }
    }
    pub fn right(&mut self) {
        let mut movement = false;
        for i in 0..SIZE {
            let mut row = [0; SIZE];
            let mut k = 0;
            for j in (0..SIZE).rev() {
                row[k] = self.board[i][j];
                k += 1;
            }
            if self.move_row(&mut row) {
                movement = true;
                let mut k = 0;
                for j in (0..SIZE).rev() {
                    self.board[i][j] = row[k];
                    k += 1
                }
            }
        }
        if movement {
            self.add_rand_block();
        }
    }
    fn move_row(&mut self, row: &mut [usize; SIZE]) -> (bool) {
        let mut movement = false;
        let mut zeros_count = 0;
        // First pass to count zeros and unite appropriate blocks
        for i in 0..SIZE {
            if row[i] == 0 {
                // Increment num zeroes
                zeros_count += 1;
            } else {
                // Check if you should combine
                for k in i + 1..SIZE {
                    // Combine if they're the same and end loop
                    if row[i] == row[k] {
                        movement = true;
                        row[i] += row[k];
                        row[k] = 0;
                        // Score increased by the sum of the combined numbers
                        self.score += row[i];
                        // Every combination increases the total number of blanks
                        self.board_zeros += 1;
                        break;
                    } else if row[k] != 0 {
                        break;
                    }
                }
            }
        }
        //Second pass to move blocks to edge
        for i in 0..SIZE - zeros_count {
            if row[i] == 0 {
                for k in i + 1..SIZE {
                    if row[k] != 0 {
                        movement = true;
                        row.swap(i, k);
                        break;
                    }
                }
            }
        }
        movement
    }
    fn add_rand_block(&mut self) {
        // 1 in 8 odds it will be a 4 instead of a 2
        let num = if xorshift128(&mut self.seed) % 8 == 0 {
            4
        } else {
            2
        };

        // Empty position to put the new block in
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
    fn game_over(&self) -> bool {
        let mut board = self.clone();
        board.left();
        if board.board != self.board {
            return false;
        }

        let mut board = self.clone();
        board.right();
        if board.board != self.board {
            return false;
        }

        let mut board = self.clone();
        board.up();
        if board.board != self.board {
            return false;
        }

        let mut board = self.clone();
        board.down();
        if board.board != self.board {
            return false;
        }

        true
    }
    pub fn current_state(&self) -> (usize, &[[usize; SIZE]; SIZE], bool) {
        (self.score, &self.board, self.game_over())
    }
}

use std::time::{SystemTime, UNIX_EPOCH};

/// Terrible `std_lib` way to get random numbers
fn rand_nanos() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}

/// State must not be all zero
fn xorshift128(state: &mut [u32; 4]) -> u32 {
    let mut t: u32 = state[3];
    t ^= t << 11;
    t ^= t << 8;
    state[3] = state[2];
    state[2] = state[1];
    state[1] = state[0];
    t ^= state[0];
    t ^= state[0] >> 19;
    state[0] = t;
    t
}

#[test]
fn test_game_over() {
    let board = Board {
        seed: [0, 0, 0, 0],
        board_zeros: 0,
        score: 1143,
        board: [
            [4, 16, 4, 2],
            [2, 4, 32, 16],
            [4, 16, 128, 2],
            [2, 4, 32, 4],
        ],
    };

    assert_eq!(board.game_over(), true);
}
