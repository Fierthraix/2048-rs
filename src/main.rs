extern crate ncurses;

mod funcs;
mod board;

use ncurses::*;
use board::Board;

fn main() {
    let mut board = Board::new();
    initscr();
    loop {
        game_turn(&mut board);
    }
}

fn game_turn(b: &mut Board) {
    print_board(&b);

    loop {
        let fdsa = getch();
        match fdsa as u8 as char {
            'w' | 'k' => b.up(),
            'a' | 'h' => b.left(),
            's' | 'j' => b.down(),
            'd' | 'l' => b.right(),
            // Escape character
            '\x1b' => {
                getch(); // Get rid of the '\x5b' ('[')
                match getch() as u8 as char {
                    'A' => b.up(),
                    'D' => b.left(),
                    'B' => b.down(),
                    'C' => b.right(),
                    _ => {
                        continue;
                    }
                }

            }
            _ => {
                continue;
            }
        }
        break;
    }
    clear();
}

fn print_board(b: &Board) {
    let (score, grid) = b.current_state();
    let mut string = String::with_capacity(10 * grid.len() * grid.len());

    string.push_str(format!("{}\n", score).as_ref());

    for i in 0..grid.len() {
        string.push_str(format!("{:?}\n", grid[i]).as_ref());
    }
    printw(string.as_ref());
}
/*
   let (score, grid) = b.current_state();
   let mut string = String::with_capacity(10 * grid.len() * grid.len());

   string.push_str(format!("Score: {}\n\n", score).as_ref());

   for i in 0..grid.len() {
   string.push_str("--------------------");
   for j in 0..grid.len() {
   let num = grid[i][j];
   let num_len = grid[i][j] / 10 + 1;
// for i in num_len push spaces and stuff
string.push_str(format!("{}\n", grid[i][j]).as_ref());
}
}
string.push_str("--------------------");
printw(string.as_ref());
*/
